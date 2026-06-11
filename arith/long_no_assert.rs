use prusti_contracts::*;

// 8-tap symmetric FIR low-pass filter (fully unrolled).
// Coefficients: [3, 12, 30, 60, 60, 30, 12, 3], sum = 210.
#[requires(norm > 0)]
fn fir8_lowpass(
    x0: i32, x1: i32, x2: i32, x3: i32,
    x4: i32, x5: i32, x6: i32, x7: i32,
    norm: i32,
) -> i32 {
    let t0 = 3 * x0;
    let t1 = 12 * x1;
    let t2 = 30 * x2;
    let t3 = 60 * x3;
    let t4 = 60 * x4;
    let t5 = 30 * x5;
    let t6 = 12 * x6;
    let t7 = 3 * x7;
    let sum01 = t0 + t1;
    let sum23 = t2 + t3;
    let sum45 = t4 + t5;
    let sum67 = t6 + t7;
    let sum_lo = sum01 + sum23;
    let sum_hi = sum45 + sum67;
    let total = sum_lo + sum_hi;
    total / norm
}

// Approximate YCbCr-to-R (BT.601, full range, integer arithmetic scaled by 1024).
// Converts a single output channel; inputs are full-range [0, 255].
fn ycbcr_to_r(y: i32, cr: i32) -> i32 {
    let y_adj = y - 16;
    let cr_adj = cr - 128;
    let y_scaled = y_adj * 1192;
    let cr_scaled = cr_adj * 1634;
    let r_raw = y_scaled + cr_scaled;
    let r_val = r_raw / 1024;
    if r_val < 0 { 0 } else if r_val > 255 { 255 } else { r_val }
}

// Approximate YCbCr-to-G (BT.601, full range).
fn ycbcr_to_g(y: i32, cb: i32, cr: i32) -> i32 {
    let y_adj = y - 16;
    let cb_adj = cb - 128;
    let cr_adj = cr - 128;
    let y_scaled = y_adj * 1192;
    let cb_scaled = cb_adj * 401;
    let cr_scaled = cr_adj * 833;
    let g_raw = y_scaled - cb_scaled - cr_scaled;
    let g_val = g_raw / 1024;
    if g_val < 0 { 0 } else if g_val > 255 { 255 } else { g_val }
}

// Approximate YCbCr-to-B (BT.601, full range).
fn ycbcr_to_b(y: i32, cb: i32) -> i32 {
    let y_adj = y - 16;
    let cb_adj = cb - 128;
    let y_scaled = y_adj * 1192;
    let cb_scaled = cb_adj * 2066;
    let b_raw = y_scaled + cb_scaled;
    let b_val = b_raw / 1024;
    if b_val < 0 { 0 } else if b_val > 255 { 255 } else { b_val }
}

// Evaluate degree-4 polynomial a*x^4 + b*x^3 + c*x^2 + d*x + e (Horner's method unrolled).
fn eval_poly4(x: i32, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let x2 = x * x;
    let x3 = x2 * x;
    let x4 = x3 * x;
    let term4 = a * x4;
    let term3 = b * x3;
    let term2 = c * x2;
    let term1 = d * x;
    let sum43 = term4 + term3;
    let sum21 = term2 + term1;
    let sum_high = sum43 + sum21;
    sum_high + e
}

fn main() {}
