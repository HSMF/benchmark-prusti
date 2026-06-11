use prusti_contracts::*;

fn rgb565_blend_50pct_check() {
    // Blend pure red (0xF800) with pure blue (0x001F) at 50% (alpha=16).
    let src: u32 = 0xF800; // R=31, G=0, B=0
    let dst: u32 = 0x001F; // R=0,  G=0, B=31
    let alpha: u32 = 16;
    let sr = (src >> 11) & 0x1F; // 31
    let sg = (src >> 5)  & 0x3F; // 0
    let sb = src & 0x1F;          // 0
    let dr = (dst >> 11) & 0x1F; // 0
    let dg = (dst >> 5)  & 0x3F; // 0
    let db = dst & 0x1F;          // 31
    let rr = (sr * alpha + dr * (32 - alpha)) / 32; // (31*16)/32 = 15
    let rg = (sg * alpha + dg * (32 - alpha)) / 32; // 0
    let rb = (sb * alpha + db * (32 - alpha)) / 32; // (31*16)/32 = 15
    let result = ((rr & 0x1F) << 11) | ((rg & 0x3F) << 5) | (rb & 0x1F);
    // (15 << 11) | 0 | 15 = 0x7800 | 0x000F = 0x780F
    assert!(result == 0x780F);
}

fn gray4_to_bin_check() {
    // Gray code 0b0111 (5 in Gray) -> binary 5 = 0b0101
    let gray: u8 = 0b0111;
    let g3 = (gray >> 3) & 1u8; // 0
    let g2 = (gray >> 2) & 1u8; // 1
    let g1 = (gray >> 1) & 1u8; // 1
    let g0 = gray & 1u8;         // 1
    let b3 = g3;       // 0
    let b2 = b3 ^ g2;  // 1
    let b1 = b2 ^ g1;  // 0
    let b0 = b1 ^ g0;  // 1
    let result = (b3 << 3) | (b2 << 2) | (b1 << 1) | b0;
    assert!(result == 5u8);
}

fn gray4_identity_check() {
    // Gray(0) == Binary(0)
    let gray: u8 = 0u8;
    let g3 = (gray >> 3) & 1u8;
    let g2 = (gray >> 2) & 1u8;
    let g1 = (gray >> 1) & 1u8;
    let g0 = gray & 1u8;
    let b3 = g3;
    let b2 = b3 ^ g2;
    let b1 = b2 ^ g1;
    let b0 = b1 ^ g0;
    let result = (b3 << 3) | (b2 << 2) | (b1 << 1) | b0;
    assert!(result == 0u8);
}

fn fnv1a_trivial_step_check() {
    // hash=1, byte=0: (1 ^ 0) * prime = prime (no wrapping, trivially verifiable).
    let h0 = 1u32;
    let h1 = (h0 ^ 0u32) * 16777619u32;
    assert!(h1 == 16777619u32);
}

fn fnv1a_step_xor_check() {
    // XOR-only step (no multiply yet): verify XOR with known byte.
    let hash = 0xFFFF_FFFFu32;
    let byte = 0xFFu32;
    let h = hash ^ byte;
    assert!(h == 0xFFFF_FF00u32);
}

fn main() {}
