from os import makedirs
from datetime import timedelta
import json
from time import time
from os import environ
from posixpath import abspath
from glob import glob
import subprocess
import argparse
import shlex
import sys
import statistics
import paths


def check_call(args, **kwargs):
    print(f"> {shlex.join(args)}", file=sys.stderr)
    return subprocess.check_call(args, **kwargs)


def build_prusti():
    check_call(
        ("./x.py", "build"),
        cwd=paths.PRUSTI_DEV_SOURCE,
        env={
            "PRUSTI_DUMP_VIPER_PROGRAM": "true",
            "PRUSTI_DUMP_DEBUG_INFO": "true",
            "PRUSTI_CHECK_OVERFLOWS": "false",
            "PRUSTI_LOG": "debug",
            **environ,
        },
    )


def prusti(file: str):
    file = abspath(file)
    try:
        check_call(
            (
                "./x.py",
                "run",
                "--bin",
                "prusti-rustc",
                "--",
                "--edition=2021",
                "-Aunused",
                file,
            ),
            timeout=300,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=paths.PRUSTI_DEV_SOURCE,
            env={
                "PRUSTI_DUMP_VIPER_PROGRAM": "true",
                "PRUSTI_DUMP_DEBUG_INFO": "true",
                "PRUSTI_CHECK_OVERFLOWS": "false",
                "PRUSTI_LOG": "debug",
                **environ,
            },
        )
        return "ok"
    except subprocess.TimeoutExpired:
        return "timeout"
    except subprocess.CalledProcessError as e:
        print("stderr")
        print(f"\x1b[31m{e.stderr}\x1b[0m")
        print("stdout")
        print(f"\x1b[33m{e.stdout}\x1b[0m")
        print()
        return "failed"


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


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("suite")
    parser.add_argument("--reps", type=int, default=5)
    parser.add_argument("--report-dir", default="measurements")
    args = parser.parse_args()
    suite = args.suite

    build_prusti()

    measurements = []
    files = glob(f"{suite}/*.rs")
    for i, file in enumerate(files):
        report = timing(lambda: prusti(file), args.reps)
        measurements.append({"file": file, **report})
        print(f"{i+1}/{len(files)} done", file=sys.stderr)
        print(f"\x1b[33m{json.dumps(measurements[-1])}\x1b[0m", file=sys.stderr)
    makedirs(args.report_dir, exist_ok=True)
    with open(f"{args.report_dir}/{int(time())}.json", "w") as f:
        json.dump(measurements, f)
    print(json.dumps(measurements))


if __name__ == "__main__":
    main()
