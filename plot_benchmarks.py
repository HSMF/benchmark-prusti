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
        for si, suite in enumerate(suites):
            color = PALETTE[si % len(PALETTE)]
            offset = (si - (len(suites) - 1) / 2) * bar_width
            means = np.zeros(len(self.benchmarks))
            stdevs = np.zeros(len(self.benchmarks))

            for i, bench in enumerate(self.benchmarks):
                means[i] = self.data.get((suite, bench), Datapoint.default()).mean
                stdevs[i] = self.data.get((suite, bench), Datapoint.default()).stddev
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

        ax.set_yscale("log")
        ax.set_xticks(x)
        ax.set_xticklabels(self.benchmarks, rotation=35, ha="right", fontsize=9)
        ax.set_ylabel("Time (ms)", fontsize=11)
        ax.legend(handles=all_handles, fontsize=9, loc="upper left")

        plt.show()

    def overall_mean(self):
        suites = self.suites()
        for suite in suites:
            means = [
                self.data[suite, bench].mean for bench in self.benches_of_suite(suite)
            ]

            print(
                f"{suite:<40} {round(np.mean(means), 1):<10} {round(np.median(means), 1):<10} {round(np.std(means), 1):<10}"
            )

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
            print(f"{suite:<40} {failures:>5} / {total}")


def suite_name(name: str):
    if name.endswith("-overflows"):
        suffix = "-overflows"
        name = name.removesuffix("-overflows")
    else:
        suffix = ""
    name = name.removesuffix("-bitwise").removesuffix("-mixed").removesuffix("-arith")
    return name + suffix


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("benchmarks", nargs="+")
    args = parser.parse_args()

    store = Store()

    for suite in args.benchmarks:
        name = suite_name(Path(suite).name)
        try:
            with open(f"{suite}/results.csv") as f:
                data = list(csv.DictReader(f))
        except FileNotFoundError:
            print(f"{suite} ({name}) has no results.csv (yet)")
            continue

        for i in data:
            store.add_datapoint(
                Path(i["File"]).stem,
                name,
                Datapoint(
                    mean=float(i["Mean [ms]"]),
                    stddev=float(i["StdDev [ms]"]),
                    outputs=int(i["Outputs"]),
                ),
            )

    store.plot()
    store.hist_medians()
    store.num_failures()
    store.overall_mean()


if __name__ == "__main__":
    main()
