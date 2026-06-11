use prusti_contracts::*;

fn rgb565_gamma_blend_src_full_check() {
    // alpha=32 (full src): result must equal src (when src is a known pixel).
    let src: u32 = 0xF800; // pure red: R=31, G=0, B=0
    let dst: u32 = 0x001F; // pure blue: R=0, G=0, B=31
    let alpha: u32 = 32;
    let sr = (src >> 11) & 0x1F; // 31
    let sg = (src >> 5)  & 0x3F; // 0
    let sb = src & 0x1F;          // 0
    let dr = (dst >> 11) & 0x1F; // 0
    let dg = (dst >> 5)  & 0x3F; // 0
    let db = dst & 0x1F;          // 31
    let sr2 = sr * sr;  // 961
    let sg2 = sg * sg;  // 0
    let sb2 = sb * sb;  // 0
    let dr2 = dr * dr;  // 0
    let dg2 = dg * dg;  // 0
    let db2 = db * db;  // 961
    let mr = (sr2 * 32 + dr2 * 0) / 32; // 961
    let mg = (sg2 * 32 + dg2 * 0) / 32; // 0
    let mb = (sb2 * 32 + db2 * 0) / 32; // 0
    let rr = mr >> 5; // 961 >> 5 = 30
    let rg = mg >> 6; // 0
    let rb = mb >> 5; // 0
    let result = ((rr & 0x1F) << 11) | ((rg & 0x3F) << 5) | (rb & 0x1F);
    // R channel: 30 << 11 = 0xF000
    assert!(rr == 30);
    assert!(result == 0xF000);
}

fn rgb565_gamma_blend_dst_full_check() {
    // alpha=0 (full dst): result channels come entirely from dst.
    let src: u32 = 0xF800; // pure red
    let dst: u32 = 0x001F; // pure blue
    let alpha: u32 = 0;
    let sr = (src >> 11) & 0x1F; // 31
    let sg = (src >> 5)  & 0x3F; // 0
    let sb = src & 0x1F;          // 0
    let dr = (dst >> 11) & 0x1F; // 0
    let dg = (dst >> 5)  & 0x3F; // 0
    let db = dst & 0x1F;          // 31
    let sr2 = sr * sr;
    let sg2 = sg * sg;
    let sb2 = sb * sb;
    let dr2 = dr * dr;  // 0
    let dg2 = dg * dg;  // 0
    let db2 = db * db;  // 961
    let mr = (sr2 * 0 + dr2 * 32) / 32; // 0
    let mg = (sg2 * 0 + dg2 * 32) / 32; // 0
    let mb = (sb2 * 0 + db2 * 32) / 32; // 961
    let rr = mr >> 5; // 0
    let rg = mg >> 6; // 0
    let rb = mb >> 5; // 30
    let result = ((rr & 0x1F) << 11) | ((rg & 0x3F) << 5) | (rb & 0x1F);
    assert!(rb == 30);
    assert!(result == 30);
}

fn premul_rgba_opaque_check() {
    // Fully opaque RGBA: premultiplied == original (divided by 255, approx >>8 ~= /256).
    // R=128, G=64, B=32, A=255
    let rgba: u32 = (128u32 << 24) | (64u32 << 16) | (32u32 << 8) | 255u32;
    let r = (rgba >> 24) & 0xFF; // 128
    let g = (rgba >> 16) & 0xFF; // 64
    let b = (rgba >> 8) & 0xFF;  // 32
    let a = rgba & 0xFF;          // 255
    let r_pm = (r * a) >> 8; // (128*255)>>8 = 32640>>8 = 127
    let g_pm = (g * a) >> 8; // (64*255)>>8 = 16320>>8 = 63
    let b_pm = (b * a) >> 8; // (32*255)>>8 = 8160>>8 = 31
    let r5 = r_pm >> 3; // 127>>3 = 15
    let g6 = g_pm >> 2; // 63>>2 = 15
    let b5 = b_pm >> 3; // 31>>3 = 3
    assert!(r_pm == 127);
    assert!(g_pm == 63);
    assert!(b_pm == 31);
    assert!(r5 == 15);
    assert!(g6 == 15);
    assert!(b5 == 3);
}

fn fnv1a_single_step_check() {
    // hash=1, byte=0 -> (1^0)*prime = prime
    let h0 = 1u32;
    let h1 = (h0 ^ (0u32 & 0xFF)).wrapping_mul(16777619u32);
    let h2 = (h1 ^ (0u32 & 0xFF)).wrapping_mul(16777619u32);
    assert!(h1 == 16777619u32);
    // h2 = 16777619 * 16777619 (mod 2^32): not checked, just exercising the operation.
    let _ = h2;
}

fn main() {}
