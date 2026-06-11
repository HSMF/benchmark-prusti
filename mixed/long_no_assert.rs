use prusti_contracts::*;

// Linearize + blend + re-quantize two RGB565 pixels.
// Channels are squared to approximate linear light, blended arithmetically,
// then sqrt-approximated via bit shifting before repacking.
// alpha in [0, 32]: 0 = full dst, 32 = full src.
#[requires(alpha <= 32)]
fn rgb565_gamma_blend(src: u32, dst: u32, alpha: u32) -> u32 {
    let sr = (src >> 11) & 0x1F;
    let sg = (src >> 5)  & 0x3F;
    let sb = src & 0x1F;
    let dr = (dst >> 11) & 0x1F;
    let dg = (dst >> 5)  & 0x3F;
    let db = dst & 0x1F;
    // Square each channel to linearise (gamma-approx).
    let sr2 = sr * sr;  // max 31^2 = 961, fits u32
    let sg2 = sg * sg;  // max 63^2 = 3969
    let sb2 = sb * sb;
    let dr2 = dr * dr;
    let dg2 = dg * dg;
    let db2 = db * db;
    // Blend in linear space.
    let mr = (sr2 * alpha + dr2 * (32 - alpha)) / 32;
    let mg = (sg2 * alpha + dg2 * (32 - alpha)) / 32;
    let mb = (sb2 * alpha + db2 * (32 - alpha)) / 32;
    // Approximate sqrt by shift (rough, but keeps the operation count high).
    let rr = mr >> 5;  // sqrt(961) ≈ 31; 961 >> 5 = 30 (good approx)
    let rg = mg >> 6;  // sqrt(3969) ≈ 63; 3969 >> 6 = 62
    let rb = mb >> 5;
    // Repack into RGB565.
    ((rr & 0x1F) << 11) | ((rg & 0x3F) << 5) | (rb & 0x1F)
}

// FNV-1a 32-bit hash of eight bytes (unrolled); each byte masked to 8 bits.
fn fnv1a_8bytes_broken(
    b0: u32, b1: u32, b2: u32, b3: u32,
    b4: u32, b5: u32, b6: u32, b7: u32,
) -> u32 {
    let h0 = 2166136261u32;
    let h1 = (h0 ^ (b0 & 0xFF)) * 16777619u32;
    let h2 = (h1 ^ (b1 & 0xFF)) * 16777619u32;
    let h3 = (h2 ^ (b2 & 0xFF)) * 16777619u32;
    let h4 = (h3 ^ (b3 & 0xFF)) * 16777619u32;
    let h5 = (h4 ^ (b4 & 0xFF)) * 16777619u32;
    let h6 = (h5 ^ (b5 & 0xFF)) * 16777619u32;
    let h7 = (h6 ^ (b6 & 0xFF)) * 16777619u32;
    let h8 = (h7 ^ (b7 & 0xFF)) * 16777619u32;
    h8
}

// FNV-1a 32-bit hash of eight bytes (unrolled); each byte masked to 8 bits.
fn fnv1a_8bytes(
    b0: u32, b1: u32, b2: u32, b3: u32,
    b4: u32, b5: u32, b6: u32, b7: u32,
) -> u32 {
    let h0 = 2166136261u32;
    let h1 = (h0 ^ (b0 & 0xFF)) .wrapping_mul (16777619u32);
    let h2 = (h1 ^ (b1 & 0xFF)) .wrapping_mul (16777619u32);
    let h3 = (h2 ^ (b2 & 0xFF)) .wrapping_mul (16777619u32);
    let h4 = (h3 ^ (b3 & 0xFF)) .wrapping_mul (16777619u32);
    let h5 = (h4 ^ (b4 & 0xFF)) .wrapping_mul (16777619u32);
    let h6 = (h5 ^ (b5 & 0xFF)) .wrapping_mul (16777619u32);
    let h7 = (h6 ^ (b6 & 0xFF)) .wrapping_mul (16777619u32);
    let h8 = (h7 ^ (b7 & 0xFF)) .wrapping_mul (16777619u32);
    h8
}

// Premultiply 8-bit RGBA channels by alpha, then pack to RGB565.
// Input packed: R in [31:24], G in [23:16], B in [15:8], A in [7:0].
fn premul_rgba_to_rgb565(rgba: u32) -> u32 {
    let r = (rgba >> 24) & 0xFF;
    let g = (rgba >> 16) & 0xFF;
    let b = (rgba >> 8) & 0xFF;
    let a = rgba & 0xFF;
    // Premultiply: ch_pm = (ch * a) >> 8  (approximate /255 via >>8)
    let r_pm = (r * a) >> 8;
    let g_pm = (g * a) >> 8;
    let b_pm = (b * a) >> 8;
    // Quantize 8-bit to 5 or 6 bits and expand for rounding.
    let r5 = r_pm >> 3;
    let g6 = g_pm >> 2;
    let b5 = b_pm >> 3;
    let r5e = (r5 & 0x1F) | ((r5 & 0x1F) >> 3); // minor expand
    let g6e = (g6 & 0x3F) | ((g6 & 0x3F) >> 6);
    let b5e = (b5 & 0x1F) | ((b5 & 0x1F) >> 3);
    ((r5e & 0x1F) << 11) | ((g6e & 0x3F) << 5) | (b5e & 0x1F)
}

fn main() {}
