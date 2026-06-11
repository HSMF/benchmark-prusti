use prusti_contracts::*;

// Align x up to the next multiple of align (align must be power of 2).
#[requires(align > 0)]
fn align_up(x: u32, align: u32) -> u32 {
    let mask = align - 1;
    (x + mask) & !mask
}

// One step of FNV-1a 32-bit hash: XOR-fold then multiply by prime.
fn fnv1a_step(hash: u32, byte: u32) -> u32 {
    let h = hash ^ (byte & 0xFF);
    h * 16777619u32
}

// Scale a 4-bit nibble (0..=15) to 8-bit range (0..=255) via multiply.
fn nibble_to_byte(x: u8) -> u8 {
    let n = x & 0x0F;
    n * 17
}

// Pack 5-bit R, 6-bit G, 5-bit B into RGB565 u32.
fn pack_rgb565(r: u32, g: u32, b: u32) -> u32 {
    ((r & 0x1F) << 11) | ((g & 0x3F) << 5) | (b & 0x1F)
}

// Quantize an 8-bit value to 5 bits, then expand back (replicate MSBs into LSBs).
fn quantize_5bit_expand(x: u8) -> u8 {
    let q = x >> 3;
    (q << 3) | (q >> 2)
}

fn main() {}
