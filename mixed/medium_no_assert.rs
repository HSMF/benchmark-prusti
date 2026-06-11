use prusti_contracts::*;

// Blend two RGB565-encoded pixels; alpha in [0, 32] (0 = all dst, 32 = all src).
#[requires(alpha <= 32)]
fn rgb565_blend(src: u32, dst: u32, alpha: u32) -> u32 {
    let sr = (src >> 11) & 0x1F;
    let sg = (src >> 5)  & 0x3F;
    let sb = src & 0x1F;
    let dr = (dst >> 11) & 0x1F;
    let dg = (dst >> 5)  & 0x3F;
    let db = dst & 0x1F;
    let rr = (sr * alpha + dr * (32 - alpha)) / 32;
    let rg = (sg * alpha + dg * (32 - alpha)) / 32;
    let rb = (sb * alpha + db * (32 - alpha)) / 32;
    ((rr & 0x1F) << 11) | ((rg & 0x3F) << 5) | (rb & 0x1F)
}

// Convert a 4-bit Gray code value to its binary equivalent.
fn gray4_to_bin(gray: u8) -> u8 {
    let g3 = (gray >> 3) & 1u8;
    let g2 = (gray >> 2) & 1u8;
    let g1 = (gray >> 1) & 1u8;
    let g0 = gray & 1u8;
    let b3 = g3;
    let b2 = b3 ^ g2;
    let b1 = b2 ^ g1;
    let b0 = b1 ^ g0;
    (b3 << 3) | (b2 << 2) | (b1 << 1) | b0
}

// FNV-1a 32-bit hash of four bytes (unrolled).
fn fnv1a_4bytes(b0: u32, b1: u32, b2: u32, b3: u32) -> u32 {
    let h0 = 2166136261u32;
    let h1 = (h0 ^ (b0 & 0xFF)) * 16777619u32;
    let h2 = (h1 ^ (b1 & 0xFF)) * 16777619u32;
    let h3 = (h2 ^ (b2 & 0xFF)) * 16777619u32;
    let h4 = (h3 ^ (b3 & 0xFF)) * 16777619u32;
    h4
}

fn main() {}
