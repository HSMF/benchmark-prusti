use prusti_contracts::*;

fn fir8_dc_passthrough_check() {
    // All-ones DC signal through the filter should give 1 when norm == sum of coefficients.
    let x = 1i32;
    let t0 = 3 * x;
    let t1 = 12 * x;
    let t2 = 30 * x;
    let t3 = 60 * x;
    let t4 = 60 * x;
    let t5 = 30 * x;
    let t6 = 12 * x;
    let t7 = 3 * x;
    let sum01 = t0 + t1;
    let sum23 = t2 + t3;
    let sum45 = t4 + t5;
    let sum67 = t6 + t7;
    let sum_lo = sum01 + sum23;
    let sum_hi = sum45 + sum67;
    let total = sum_lo + sum_hi; // 3+12+30+60+60+30+12+3 = 210
    let result = total / 210;
    assert!(total == 210);
    assert!(result == 1);
}

fn ycbcr_neutral_chroma_r_check() {
    // Y=235, Cb=128, Cr=128 (neutral chroma, max luma) -> R near white.
    let y = 235i32;
    let cr = 128i32;
    let y_adj = y - 16;   // 219
    let cr_adj = cr - 128; // 0
    let y_scaled = y_adj * 1192;  // 261048
    let cr_scaled = cr_adj * 1634; // 0
    let r_raw = y_scaled + cr_scaled; // 261048
    let r_val = r_raw / 1024;    // 254
    let result = if r_val < 0 { 0 } else if r_val > 255 { 255 } else { r_val };
    assert!(result == 254);
}

fn ycbcr_black_check() {
    // Y=16 (black), neutral chroma -> R=0.
    let y = 16i32;
    let cr = 128i32;
    let y_adj = y - 16;    // 0
    let cr_adj = cr - 128; // 0
    let r_raw = y_adj * 1192 + cr_adj * 1634; // 0
    let r_val = r_raw / 1024; // 0
    let result = if r_val < 0 { 0 } else if r_val > 255 { 255 } else { r_val };
    assert!(result == 0);
}

fn poly4_eval_check() {
    // x^4 - 2x^3 + x^2 + x - 1 at x=2: 16 - 16 + 4 + 2 - 1 = 5.
    let x = 2i32;
    let x2 = x * x;   // 4
    let x3 = x2 * x;  // 8
    let x4 = x3 * x;  // 16
    let term4 = 1 * x4;  // 16
    let term3 = -2 * x3; // -16
    let term2 = 1 * x2;  // 4
    let term1 = 1 * x;   // 2
    let sum43 = term4 + term3; // 0
    let sum21 = term2 + term1; // 6
    let result = sum43 + sum21 + (-1i32); // 5
    assert!(result == 5);
}

fn poly4_zero_check() {
    // All coefficients 0 -> always 0.
    let x = 7i32;
    let x2 = x * x;
    let x3 = x2 * x;
    let x4 = x3 * x;
    let result = 0 * x4 + 0 * x3 + 0 * x2 + 0 * x + 0;
    assert!(result == 0);
}

fn main() {}
