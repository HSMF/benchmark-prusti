from typing import Collection
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

ARITH = [
    "alpha_blend_50pct_check.vpr",
    "alpha_blend_channel.vpr",
    "alpha_blend_opaque_check.vpr",
    "bt601_luma_pure_red_check.vpr",
    "bt601_luma.vpr",
    "bt601_luma_white_check.vpr",
    "celsius_to_fahrenheit_x10.vpr",
    "clamp_lower_check.vpr",
    "clamp_upper_check.vpr",
    "clamp.vpr",
    "div_ceil_check.vpr",
    "div_ceil.vpr",
    "eval_poly4.vpr",
    "eval_quadratic.vpr",
    "fahrenheit_check.vpr",
    "fir8_dc_passthrough_check.vpr",
    "fir8_lowpass.vpr",
    "linear_map_check.vpr",
    "linear_map.vpr",
    "midpoint_check.vpr",
    "midpoint.vpr",
    "poly4_eval_check.vpr",
    "poly4_zero_check.vpr",
    "quadratic_root_check.vpr",
    "sat_add_no_overflow_check.vpr",
    "sat_add_overflow_check.vpr",
    "sat_add.vpr",
    "weighted_avg_3.vpr",
    "ycbcr_black_check.vpr",
    "ycbcr_neutral_chroma_r_check.vpr",
    "ycbcr_to_b.vpr",
    "ycbcr_to_g.vpr",
    "ycbcr_to_r.vpr",
]

BITWISE = [
    "bit_clear_check.vpr",
    "bit_clear.vpr",
    "bit_set_check.vpr",
    "bit_set.vpr",
    "bit_test_check.vpr",
    "bit_test.vpr",
    "bit_toggle_check.vpr",
    "bit_toggle.vpr",
    "bswap_u32_check.vpr",
    "bswap_u32.vpr",
    "has_any_flag_check.vpr",
    "has_any_flag.vpr",
    "pack_le_check.vpr",
    "pack_u8x4_le.vpr",
    "parity_three_bits_check.vpr",
    "parity_u32.vpr",
    "parity_zero_check.vpr",
    "reverse_bits_u16_known_check.vpr",
    "reverse_bits_u16_palindrome_check.vpr",
    "reverse_bits_u16.vpr",
    "reverse_bits_u8_check.vpr",
    "reverse_bits_u8.vpr",
    "rgba_mask_channels.vpr",
    "rgba_mask_identity_check.vpr",
    "swap_nibbles_check.vpr",
    "swap_nibbles.vpr",
    "xor_checksum_idempotent_check.vpr",
    "xor_checksum_u32.vpr",
]

MIXED = [
    "align_up_already_aligned_check.vpr",
    "align_up_check.vpr",
    "align_up.vpr",
    "fnv1a_4bytes.vpr",
    "fnv1a_8bytes_broken.vpr",
    "fnv1a_8bytes.vpr",
    "fnv1a_single_step_check.vpr",
    "fnv1a_step.vpr",
    "fnv1a_step_xor_check.vpr",
    "fnv1a_trivial_step_check.vpr",
    "gray4_identity_check.vpr",
    "gray4_to_bin_check.vpr",
    "gray4_to_bin.vpr",
    "nibble_to_byte_max_check.vpr",
    "nibble_to_byte.vpr",
    "nibble_to_byte_zero_check.vpr",
    "pack_rgb565_blue_check.vpr",
    "pack_rgb565_red_check.vpr",
    "pack_rgb565.vpr",
    "premul_rgba_opaque_check.vpr",
    "premul_rgba_to_rgb565.vpr",
    "quantize_5bit_expand.vpr",
    "quantize_5bit_max_check.vpr",
    "rgb565_blend_50pct_check.vpr",
    "rgb565_blend.vpr",
    "rgb565_gamma_blend_dst_full_check.vpr",
    "rgb565_gamma_blend_src_full_check.vpr",
    "rgb565_gamma_blend.vpr",
]


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


def by_benchmark(d: Data, bench: Collection[str]) -> Data:
    return {k: v for k, v in d.items() if v.file in bench}


def bitwise(d: Data) -> Data:
    return by_benchmark(d, BITWISE)


def mixed(d: Data) -> Data:
    return by_benchmark(d, MIXED)


def arith(d: Data) -> Data:
    return by_benchmark(d, ARITH)


def mean(d: Data) -> float | None:
    if len(d) == 0:
        return None
    return float(np.mean([i.mean for i in d.values()]))


def median(d: Data) -> float | None:
    if len(d) == 0:
        return None
    return float(np.median([i.mean for i in d.values()]))


def stddev(d: Data) -> float | None:
    if len(d) == 0:
        return None
    return float(np.std([i.mean for i in d.values()]))


def deep_get(d: dict, *path: str) -> dict:
    for p in path:
        d[p] = d.get(p, {})
        d = d[p]

    return d


def ser_data(data: dict[str, Datapoint]):
    return {
        k: {
            "mean": v.mean,
            "timeout": v.best > TIMEOUT,
            "fails": v.outputs > 0,
            "stddev": v.stddev,
        }
        for k, v in data.items()
    }


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

        def individual_stats(data: Data):
            if len(data) == 0:
                return None
            return {
                "mean_no_timeout": mean(minus(data, any_timeout(data))),
                "median_no_timeout": median(minus(data, any_timeout(data))),
                "stddev_no_timeout": stddev(minus(data, any_timeout(data))),
                "timeout": len(timeouts(data)),
                "fail": len(failures(data)),
            }

        part["timeout"] = timeout
        part["any_timeout"] = len(any_timeout(data))
        part["fail"] = fail
        part["total"] = total
        part["mean_no_timeout"] = mean(minus(data, any_timeout(data)))
        part["mixed_mean_no_timeout"] = mean(minus(mixed(data), any_timeout(data)))
        part["arith_mean_no_timeout"] = mean(minus(arith(data), any_timeout(data)))
        part["bitwise_mean_no_timeout"] = mean(minus(bitwise(data), any_timeout(data)))
        part["arith"] = individual_stats(arith(data))
        part["bitwise"] = individual_stats(bitwise(data))
        part["mixed"] = individual_stats(mixed(data))
        part["mean_no_timeout"] = np.mean(
            [i.mean for i in minus(data, any_timeout(data)).values()]
        )
        part["data"] = ser_data(data)

    with (
        open(args.output, "w")
        if args.output is not None
        else contextlib.nullcontext(sys.stdout)
    ) as f:
        json.dump(ret, indent=2, fp=f)


if __name__ == "__main__":
    main()
