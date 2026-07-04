import json
import subprocess
import itertools
from typing import Callable
from typing import Iterable
from dataclasses import dataclass
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import numpy as np
from pathlib import Path
import csv
import argparse

PALETTE = [
    "#4C72B0",
    "#DD8452",
    "#55A868",
    "#C44E52",
    "#8172B3",
    "#937860",
    "#DA8BC3",
    "#8C8C8C",
]


@dataclass
class Datapoint:
    mean: float
    stddev: float
    outputs: int

    @classmethod
    def default(cls):
        return cls(mean=0.0, stddev=0.0, outputs=0)


class Store:
    def __init__(self):
        self.benchmarks: list[str] = []
        # suite, benchmark -> data
        self.data: dict[tuple[str, str], Datapoint] = {}

    def add_datapoint(self, label: str, suite: str, datapoint: Datapoint):
        if label not in self.benchmarks:
            self.benchmarks.append(label)

        self.data[suite, label] = datapoint

    def filter(self, f: Callable[[Datapoint], bool], strict=False):
        suites = self.suites()
        ret = Store()
        comb = all if strict else any
        for b in self.benchmarks:
            datapoints = [self.data.get((s, b), None) for s in suites]
            datapoints = [(s, i) for s, i in zip(suites, datapoints) if i is not None]
            keep = comb(f(d) for _, d in datapoints)
            if not keep:
                continue
            for s, data in datapoints:
                ret.add_datapoint(b, s, data)
        return ret

    def suites(self):
        return sorted({suite for suite, _ in self.data})

    def benches_of_suite(self, suite: str) -> Iterable[str]:
        return [bench for s, bench in self.data if s == suite]

    def plot(self):
        suites = self.suites()
        print(suites)
        fig, ax = plt.subplots()
        x = np.arange(len(self.benchmarks))
        all_handles = []

        bar_width = 0.75 / len(suites)  # total group width = 0.75
        error_x = []
        for si, suite in enumerate(suites):
            color = PALETTE[si % len(PALETTE)]
            offset = (si - (len(suites) - 1) / 2) * bar_width
            means = np.zeros(len(self.benchmarks))
            stdevs = np.zeros(len(self.benchmarks))

            for i, bench in enumerate(self.benchmarks):
                dp = self.data.get((suite, bench))
                if dp is None:
                    error_x.append(x[i] + offset)
                    continue
                means[i] = dp.mean
                stdevs[i] = dp.stddev
                if dp.outputs != 0:
                    error_x.append(x[i] + offset)
            all_handles.append(mpatches.Patch(color=color, label=suite))

            bars = ax.bar(
                x + offset,
                means,
                width=bar_width * 0.9,
                color=color,
                alpha=0.85,
                label=suite,
                zorder=3,
            )
            ax.errorbar(
                x + offset,
                means,
                yerr=stdevs,
                fmt="none",
                ecolor="black",
                elinewidth=1.2,
                capsize=4,
                zorder=4,
            )
        if error_x:
            ax.scatter(
                error_x,
                [0] * len(error_x),
                marker="x",
                color="red",
                s=40,
                linewidths=1.5,
                zorder=5,
                clip_on=False,
                transform=ax.get_xaxis_transform(),
            )
        ax.set_yscale("log")
        ax.set_xticks(x)
        ax.set_xticklabels(self.benchmarks, rotation=35, ha="right", fontsize=9)
        ax.set_ylabel("Time (ms)", fontsize=11)
        ax.legend(handles=all_handles, fontsize=9, loc="upper left")

        plt.savefig("/tmp/t.png")
        subprocess.run(("kitty", "+kitten", "icat", "/tmp/t.png"))

    def overall_mean(self):
        suites = self.suites()
        ret = []
        for suite in suites:
            means = [
                self.data[suite, bench].mean for bench in self.benches_of_suite(suite)
            ]

            mean = round(np.mean(means), 1)
            median = round(np.median(means), 1)
            stddev = round(np.std(means), 1)
            ret.append(
                {"suite": suite, "mean": mean, "median": median, "stddev": stddev}
            )
            print(f"{suite:<40} {mean:<10} {median:<10} {stddev:<10}")
        return ret

    def hist_medians(self):
        suites = self.suites()
        width = min(2, len(suites))
        height = (len(suites) + 2) // 2
        fig, axes = plt.subplots(height, width)

        max_x = 0
        for suite, ax in zip(suites, axes.flatten()):
            means = [
                self.data[suite, bench].mean for bench in self.benches_of_suite(suite)
            ]
            ax.set_title(suite)
            _, x, _ = ax.hist(means)
            max_x = max(max_x, np.max(x))
        for ax in axes.flatten():
            ax.set_xlim(xmin=0, xmax=max_x)

        plt.show()

    def num_failures(self):
        suites = self.suites()
        ret = []
        for suite in suites:
            failures = 0
            total = 0
            for bench in self.benchmarks:
                if (suite, bench) not in self.data:
                    continue
                d = self.data[suite, bench]
                total += 1
                if d.outputs != 0:
                    failures += 1
            ret.append({"suite": suite, "failures": failures, "total": total})
            print(f"{suite:<40} {failures:>5} / {total}")
        return ret


def suite_name(name: str):
    if name.endswith("-overflows"):
        suffix = "-overflows"
        name = name.removesuffix("-overflows")
    else:
        suffix = ""
    name = name.removesuffix("-bitwise").removesuffix("-mixed").removesuffix("-arith")
    return name + suffix


def load_data(suite: str, store: Store, data: list[dict]):
    for i in data:
        store.add_datapoint(
            Path(i["File"]).stem,
            suite,
            Datapoint(
                mean=float(i["Mean [ms]"]),
                stddev=float(i["StdDev [ms]"]),
                outputs=int(i["Outputs"]),
            ),
        )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("benchmarks", nargs="+")
    parser.add_argument("--prover", nargs="*", default=["z3"])
    parser.add_argument("--filter")
    parser.add_argument("--output-stats")
    args = parser.parse_args()

    store = Store()

    for suite, prover in itertools.product(args.benchmarks, args.prover):
        name = suite_name(Path(suite).name)
        if len(args.prover) != 1:
            name += f"-{prover}"
        file = f"{suite}/results-{prover}.csv"
        try:
            with open(file) as f:
                data = list(csv.DictReader(f))
        except FileNotFoundError:
            print(f"{suite} ({name}) has no {file} (yet)")
            continue
        load_data(name, store, data)

    if args.filter is not None:
        store = store.filter(lambda d: eval(args.filter))
    # store = store.filter(lambda d: d.outputs == 0)
    # store = store.filter(lambda d: d.outputs != 0)
    store.plot()
    # store.hist_medians()
    failures = store.num_failures()
    means = store.overall_mean()

    if args.output_stats is not None:
        with open(args.output_stats, "w") as f:
            json.dump({"failures": failures, "means": means}, f)


if __name__ == "__main__":
    main()
