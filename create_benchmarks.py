from textwrap import dedent
from paths import PRUSTI_DEV_SOURCE
from random import choice
from random import randint
import subprocess
from typing import Iterable
import signal
from subprocess import Popen
import shutil
import itertools
from pathlib import Path
from os import makedirs
from datetime import timedelta
import json
from time import time
from os import environ
import chop
from glob import glob
import subprocess
import argparse
import shlex
import sys
import statistics
import paths

TMPDIR = "/tmp/prusti-bench"
DOCKER_IMAGE = None


def get_file(name: str):
    tmpdir = Path(TMPDIR)
    stem = Path(name).stem
    suffix = Path(name).suffix
    candidates = itertools.chain(
        (tmpdir / name,), (tmpdir / f"{stem}-{i}{suffix}" for i in itertools.count(1))
    )
    for cand in candidates:
        if not cand.exists():
            return cand
    raise Exception()


def check_call(args, **kwargs):
    print(f"> {shlex.join(args)}", file=sys.stderr)
    return subprocess.check_call(args, **kwargs)


def run(args, **kwargs):
    print(f"> {shlex.join(args)}", file=sys.stderr)
    return subprocess.run(args, **kwargs)


def build_prusti():
    check_call(
        ("./x.py", "build"),
        cwd=paths.PRUSTI_DEV_SOURCE,
    )


def start_viperserver():
    argv = (
        "java",
        "-Xss128m",
        "-Xmx2048m",
        "-jar",
        paths.VIPERSERVER,
        "--port",
        "6951",
    )
    print(f"> {shlex.join(argv)}")
    return Popen(argv)


def compile_to_viper(file: Path, check_overflows):
    makedirs(TMPDIR, exist_ok=True)
    check_call(
        (
            "./x.py",
            "run",
            "--bin",
            "prusti-rustc",
            "--",
            "--edition=2021",
            "-Aunused",
            str(file),
        ),
        env={
            "PRUSTI_DUMP_VIPER_PROGRAM": "true",
            "PRUSTI_DUMP_DEBUG_INFO": "true",
            "PRUSTI_CHECK_OVERFLOWS": "true" if check_overflows else "false",
            "PRUSTI_LOG": "debug",
            "PRUSTI_PRINT_HASH": "true",
            **environ,
        },
        cwd=PRUSTI_DEV_SOURCE,
    )

    dst = file.with_suffix(".vpr")

    shutil.copy(PRUSTI_DEV_SOURCE + "/log/viper_program/program-check.vpr", dst)
    return dst


class DockerCompiler:
    def __init__(self):
        self.project = get_file("proj")
        (self.project / "src").mkdir(parents=True, exist_ok=True)
        makedirs(TMPDIR, exist_ok=True)

    def compile(self, file: Path, check_overflows):
        cargo_toml = self.project / "Cargo.toml"
        cargo_toml.write_text(dedent("""
        [package]
        name = "bad-example"
        version = "0.1.0"
        edition = "2024"

        [dependencies]
        prusti-contracts = { path = "/opt/prusti-dev/prusti-contracts/prusti-contracts" }
        """))
        src = self.project / "src" / "lib.rs"
        src.write_text(file.read_text())
        check_overflows = str(check_overflows).lower()

        viperfile = self.project / "target/verify/log/viper_program/program-check.vpr"
        viperfile.unlink(missing_ok=True)

        run(
            (
                "docker",
                "run",
                "--rm",
                "-it",
                "-e",
                "PRUSTI_DUMP_VIPER_PROGRAM=true",
                "-e",
                "PRUSTI_DUMP_DEBUG_INFO=true",
                "-e",
                f"PRUSTI_CHECK_OVERFLOWS={check_overflows}",
                "-w",
                "/root/project",
                "-v",
                f"{self.project}:/root/project",
                DOCKER_IMAGE,
                "bash",
                "-c",
                """
                useradd user -u 1000
                trap 'chown -R user:user .' EXIT
                cargo prusti
                """,
            )
        )

        dst = file.with_suffix(".vpr")

        if viperfile.exists():
            shutil.copy(viperfile, dst)

        return dst


def compute_stats(measurements):
    try:
        stdev = statistics.stdev(measurements)
    except statistics.StatisticsError:
        stdev = 0
    median = statistics.median(measurements)
    mean = statistics.mean(measurements)
    return {
        "measurements": measurements,
        "mean": mean,
        "median": median,
        "stdev": stdev,
        "min": min(measurements),
        "max": max(measurements),
    }


def timing(f, n):
    measurements = []
    for i in range(n):
        start = time()
        f()
        end = time()
        duration = end - start
        measurements.append(duration)

        time_elapsed = sum(measurements)
        time_per_iter = time_elapsed / (i + 1)
        iters_left = n - 1 - i
        time_left = time_per_iter * iters_left
        print(
            f"estimated time left for measurement {timedelta(seconds=round(time_left, 2))}",
            file=sys.stderr,
        )
    return compute_stats(measurements)


def benchmarks(suite: str) -> Iterable[tuple[str, str]]:
    """
    returns (filename, src)
    """
    files = glob(f"{suite}/*.rs")
    for i, file in enumerate(files):
        with open(file) as f:
            code = f.read()
        yield from chop.chop(code)


def iterative_bench(n: int, ops: list[str], step=1) -> Iterable[tuple[str, str]]:
    """
    returns (filename, src)
    """
    initial = randint(0, 2**16)
    body = []

    test = get_file("test.rs")
    test_exe = get_file("test")

    def wrap_func(body: list[str], name, check=False):
        str_body = "".join(i + "\n" for i in body)
        annotation = (
            f"use prusti_contracts::*;\n#[requires(x == {initial})]\n" if check else ""
        )
        return (
            annotation
            + f"fn {name}(x: i64) -> i64 "
            + "{\n"
            + str_body
            + "    return x;\n}"
        )

    num_ops = 0
    while num_ops < n:
        val = randint(0, 2**16)
        op = choice(ops)
        body.append(f"let x = x {op} {val};")

        test.write_text(wrap_func(body, "foo") + f"""
        fn main() {{
            let ret = foo({initial});
            println!("{{}}", ret);
        }}
        """)

        try:
            check_call(
                ("rustc", str(test), "-o", str(test_exe)), stderr=subprocess.DEVNULL
            )
        except subprocess.CalledProcessError:
            body.pop()
            continue

        r = run((str(test_exe),), stdout=subprocess.PIPE)
        if r.returncode != 0:
            body.pop()
            continue
        try:
            expected = int(r.stdout.decode())
        except ValueError:
            continue

        num_ops += 1
        if num_ops % step != 0:
            continue
        body.append(f"assert_eq!(x, {expected});")

        name = f"f{num_ops}"
        yield name, wrap_func(body, name, check=True)


def compile_suite(benchmarks, check_overflows):
    f = get_file("benchmarks")
    makedirs(f)
    d = DockerCompiler()
    for filename, src in benchmarks:
        rsfile = f / f"{filename}.rs"
        rsfile.write_text(src)
        if DOCKER_IMAGE is None:
            viper = compile_to_viper(rsfile, check_overflows)
        else:
            viper = d.compile(rsfile, check_overflows)
    return f


def run_benchmark(dir):
    output = get_file("results.csv")
    check_call(
        ("utils/scripts/benchmark.sh", str(dir), "--csv-file", str(output)),
        cwd=paths.SILICON_SOURCE,
    )

    return output


def main():
    global DOCKER_IMAGE
    parser = argparse.ArgumentParser()
    parser.add_argument("suite")
    parser.add_argument("--reps", type=int, default=5)
    parser.add_argument("--report-dir", default="measurements")
    parser.add_argument("--check-overflows", action="store_true")
    parser.add_argument("--output")
    parser.add_argument("--docker-image")
    args = parser.parse_args()
    suite = args.suite

    DOCKER_IMAGE = args.docker_image

    if DOCKER_IMAGE is None:
        build_prusti()
    # dir = compile_suite(
    #     iterative_bench(100, ["+", "-", "/", "*"], step=10), args.check_overflows
    # )
    dir = compile_suite(benchmarks(suite), args.check_overflows)
    if args.output is None:
        print(dir)
    else:
        if Path(args.output).exists():
            shutil.rmtree(args.output)
        shutil.move(dir, args.output)
    # run_benchmark(dir)


if __name__ == "__main__":
    main()
