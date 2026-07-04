from datetime import timedelta
from time import time
from typing import Callable
import itertools
from os import makedirs
from pathlib import Path
import shutil
import os
from posixpath import abspath
import paths
from create_benchmarks import run
from create_benchmarks import get_file
import argparse
import csv


def run_benchmark(dir, docker: bool = False, reps: int = 10, prover="z3"):
    output = get_file("results.csv")
    output.write_text("")
    if docker:
        o = run(
            (
                "docker",
                "run",
                "--rm",
                "--name",
                "silicon",
                "-v",
                f"{dir}:/tmp/benches",
                "-v",
                f"{output}:/tmp/output",
                "silicon:latest",
                "bash",
                "-c",
                f"""
                mkdir /tmp/benchmark
                cp -r /tmp/benches/. /tmp/benchmark
                chown -R root /tmp/benchmark
                chown root /tmp/output
                utils/scripts/benchmark.sh /tmp/benches --csv-file /tmp/t --repetitions {reps} --prover {prover}
                cp /tmp/t /tmp/output
                chown ubuntu /tmp/output
                """,
            )
        )
        o.check_returncode()
    else:
        run(
            ("utils/scripts/benchmark.sh", str(dir), "--csv-file", str(output)),
            cwd=paths.SILICON_SOURCE,
        )

    return output


def next_results_file(dir: Path):
    for i in itertools.count(1):
        cand = dir / f"results-{i}.csv"
        if not cand.exists():
            return cand
    raise Exception()


def run_full_suites(
    suites, docker, reps, get_results_file: Callable[[Path], Path], prover
):
    print(suites)
    for suite in suites:
        t0 = time()
        results = get_results_file(Path(suite))
        ret = run_benchmark(abspath(suite), docker, reps, prover)
        makedirs(results.parent, exist_ok=True)
        shutil.copy(ret, results)
        t1 = time()
        print("==============")
        print(f"done with {suite} in {timedelta(seconds=int(t1-t0))}")
        print("==============")
        print()


def main():
    makedirs("/tmp/prusti-bench", exist_ok=True)
    parser = argparse.ArgumentParser()
    parser.add_argument("suites", nargs="+")
    parser.add_argument("--docker", action="store_true")
    parser.add_argument("--reps", default=10)
    parser.add_argument(
        "--cycle", action="store_true", help="continually repeat the benchmark"
    )
    parser.add_argument("--prover", default="Z3")
    args = parser.parse_args()

    if args.cycle:
        while True:
            run_full_suites(
                args.suites,
                args.docker,
                args.reps,
                lambda s: next_results_file(s / "results"),
                args.prover,
            )
    else:
        run_full_suites(
            args.suites,
            args.docker,
            args.reps,
            lambda s: s / f"results-{args.prover.lower()}.csv",
            args.prover,
        )


if __name__ == "__main__":
    main()
