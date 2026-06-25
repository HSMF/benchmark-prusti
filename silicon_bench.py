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


def run_benchmark(dir):
    output = get_file("results.csv")
    run(
        ("utils/scripts/benchmark.sh", str(dir), "--csv-file", str(output)),
        cwd=paths.SILICON_SOURCE,
    )

    return output


def main():
    makedirs("/tmp/prusti-bench", exist_ok=True)
    parser = argparse.ArgumentParser()
    parser.add_argument("suites", nargs="+")
    args = parser.parse_args()

    for suite in args.suites:
        ret = run_benchmark(abspath(suite))
        shutil.copy(ret, Path(suite) / "results.csv")


if __name__ == "__main__":
    main()
