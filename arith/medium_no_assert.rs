use prusti_contracts::*;

// Linearly map x from [in_lo, in_hi] to [out_lo, out_hi].
#[requires(in_hi > in_lo)]
#[requires(x >= in_lo && x <= in_hi)]
fn linear_map(x: i32, in_lo: i32, in_hi: i32, out_lo: i32, out_hi: i32) -> i32 {
    let num = (x - in_lo) * (out_hi - out_lo);
    let den = in_hi - in_lo;
    out_lo + num / den
}

// BT.601 approximate luminance from 8-bit RGB (coefficients scaled by 1000).
#[requires(r <= 255 && g <= 255 && b <= 255)]
fn bt601_luma(r: u32, g: u32, b: u32) -> u32 {
    let r_part = 299 * r;
    let g_part = 587 * g;
    let b_part = 114 * b;
    let weighted = r_part + g_part + b_part;
    weighted / 1000
}

// Evaluate quadratic polynomial ax^2 + bx + c.
fn eval_quadratic(x: i32, a: i32, b: i32, c: i32) -> i32 {
    let x_sq = x * x;
    let term_a = a * x_sq;
    let term_b = b * x;
    term_a + term_b + c
}

// Weighted average of three values; wa, wb, wc must sum to 1024.
#[requires(wa + wb + wc == 1024)]
fn weighted_avg_3(a: i32, b: i32, c: i32, wa: i32, wb: i32, wc: i32) -> i32 {
    let sum = a * wa + b * wb + c * wc;
    sum / 1024
}

// Alpha-blend one channel: (src * alpha + dst * (256 - alpha)) / 256.
// alpha in [0, 256], channels in [0, 255].
#[requires(alpha <= 256 && src <= 255 && dst <= 255)]
fn alpha_blend_channel(src: u32, dst: u32, alpha: u32) -> u32 {
    let src_contrib = src * alpha;
    let dst_contrib = dst * (256 - alpha);
    (src_contrib + dst_contrib) / 256
}

fn main() {}
