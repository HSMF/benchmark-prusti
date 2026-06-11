import argparse
import csv
import matplotlib.pyplot as plt


def parse_filename(filename: str):
    return int(filename.removeprefix("f").removesuffix(".vpr"))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("benchmarks")
    args = parser.parse_args()

    with open(args.benchmarks) as f:
        data = list(csv.DictReader(f))

    x = [parse_filename(i["File"]) for i in data]
    mean = [float(i["Mean [ms]"]) for i in data]

    plt.scatter(x, mean)
    plt.show()


if __name__ == "__main__":
    main()
