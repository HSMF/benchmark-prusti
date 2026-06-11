use prusti_contracts::*;

fn linear_map_check() {
    // Map 5 from [0, 10] to [0, 100].
    let x = 5i32;
    let in_lo = 0i32;
    let in_hi = 10i32;
    let out_lo = 0i32;
    let out_hi = 100i32;
    let num = (x - in_lo) * (out_hi - out_lo); // 5 * 100 = 500
    let den = in_hi - in_lo;                   // 10
    let result = out_lo + num / den;            // 50
    assert!(result == 50);
}

fn bt601_luma_pure_red_check() {
    // Pure red at full intensity: Y = 299 * 255 / 1000 = 76.
    let r = 255u32;
    let g = 0u32;
    let b = 0u32;
    let weighted = 299 * r + 587 * g + 114 * b; // 76245
    let result = weighted / 1000;
    assert!(result == 76);
}

fn bt601_luma_white_check() {
    // Pure white: Y = (299 + 587 + 114) * 255 / 1000 = 255.
    let r = 255u32;
    let g = 255u32;
    let b = 255u32;
    let weighted = 299 * r + 587 * g + 114 * b; // 255000
    let result = weighted / 1000;
    assert!(result == 255);
}

fn quadratic_root_check() {
    // x^2 - x - 2 at x=2: 4 - 2 - 2 = 0.
    let x = 2i32;
    let x_sq = x * x;   // 4
    let result = x_sq - x - 2;
    assert!(result == 0);
}

fn alpha_blend_50pct_check() {
    // 50% blend of 200 (src) and 100 (dst): expect 150.
    let src = 200u32;
    let dst = 100u32;
    let alpha = 128u32;
    let src_contrib = src * alpha;          // 25600
    let dst_contrib = dst * (256 - alpha);  // 12800
    let result = (src_contrib + dst_contrib) / 256; // 150
    assert!(result == 150);
}

fn alpha_blend_opaque_check() {
    // alpha=256 is fully opaque: output == src.
    let src = 173u32;
    let dst = 42u32;
    let alpha = 256u32;
    let src_contrib = src * alpha;
    let dst_contrib = dst * (256 - alpha); // 0
    let result = (src_contrib + dst_contrib) / 256;
    assert!(result == 173);
}

fn main() {}
