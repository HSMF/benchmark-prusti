import numpy as np
import sys
import contextlib
import json
from plot_benchmarks import Datapoint
from dataclasses import field
from typing import Self
from dataclasses import dataclass
import argparse
from plot_benchmarks import load_all

TIMEOUT = (3 * 60 - 10) * 1000
type Data = dict[str, Datapoint]


@dataclass
class Suite:
    name: str
    overflows: bool
    prover: str
    id: str = field(repr=False)

    @classmethod
    def parse(cls, s: str) -> Self:
        *parts, prover = s.split("-")
        if parts[-1] == "overflows":
            parts = parts[:-1]
            overflows = True
        else:
            overflows = False
        return Suite(name="-".join(parts), overflows=overflows, prover=prover, id=s)


def timeouts(data: Data) -> Data:
    return {i: v for i, v in data.items() if v.best > TIMEOUT}


def any_timeout(data: Data) -> Data:
    return {i: v for i, v in data.items() if v.worst > TIMEOUT}


def failures(data: Data) -> Data:
    return {i: v for i, v in data.items() if v.outputs > 0}


def minus(a: Data, b: Data) -> Data:
    return {k: v for k, v in a.items() if k not in b}


def deep_get(d: dict, *path: str) -> dict:
    for p in path:
        d[p] = d.get(p, {})
        d = d[p]

    return d


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("benchmarks", nargs="+")
    parser.add_argument("--output", "-o")
    args = parser.parse_args()
    prover = ["z3", "cvc5"]

    m = [i for i in args.benchmarks if not i.endswith(".bak")]
    store = load_all(m, prover)
    suites = [Suite.parse(s) for s in store.suites()]

    ret = {}

    for suite in suites:
        data = store.data_of_suite(suite.id)
        timeout = len(timeouts(data))
        fail = len(failures(data))
        total = len(data)

        part = deep_get(
            ret,
            suite.name,
            suite.prover,
            "overflow" if (suite.overflows) else "no_overflow",
        )
        part["timeout"] = timeout
        part["any_timeout"] = len(any_timeout(data))
        part["fail"] = fail
        part["total"] = total
        part["mean_no_timeout"] = np.mean(
            [i.mean for i in minus(data, any_timeout(data)).values()]
        )
    with (
        open(args.output, "w")
        if args.output is not None
        else contextlib.nullcontext(sys.stdout)
    ) as f:
        json.dump(ret, indent=2, fp=f)


if __name__ == "__main__":
    main()
