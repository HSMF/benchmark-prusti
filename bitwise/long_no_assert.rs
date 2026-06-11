use prusti_contracts::*;

// Reverse all 16 bits of a u16 (fully unrolled).
fn reverse_bits_u16(x: u16) -> u16 {
    let b0  = (x >> 15) & 1u16;
    let b1  = (x >> 14) & 1u16;
    let b2  = (x >> 13) & 1u16;
    let b3  = (x >> 12) & 1u16;
    let b4  = (x >> 11) & 1u16;
    let b5  = (x >> 10) & 1u16;
    let b6  = (x >> 9)  & 1u16;
    let b7  = (x >> 8)  & 1u16;
    let b8  = (x >> 7)  & 1u16;
    let b9  = (x >> 6)  & 1u16;
    let b10 = (x >> 5)  & 1u16;
    let b11 = (x >> 4)  & 1u16;
    let b12 = (x >> 3)  & 1u16;
    let b13 = (x >> 2)  & 1u16;
    let b14 = (x >> 1)  & 1u16;
    let b15 = x & 1u16;
    b0 | (b1 << 1)  | (b2 << 2)  | (b3 << 3)
       | (b4 << 4)  | (b5 << 5)  | (b6 << 6)  | (b7 << 7)
       | (b8 << 8)  | (b9 << 9)  | (b10 << 10) | (b11 << 11)
       | (b12 << 12) | (b13 << 13) | (b14 << 14) | (b15 << 15)
}

// Unpack RGBA8888 (big-endian: R in [31:24]), apply per-channel bit mask, repack.
fn rgba_mask_channels(packed: u32, rmask: u32, gmask: u32, bmask: u32) -> u32 {
    let r = (packed >> 24) & 0xFF;
    let g = (packed >> 16) & 0xFF;
    let b = (packed >> 8)  & 0xFF;
    let a = packed & 0xFF;
    let r_out = r & rmask;
    let g_out = g & gmask;
    let b_out = b & bmask;
    let r_exp = r_out | (r_out >> 4); // expand masked nibble to fill byte
    let g_exp = g_out | (g_out >> 4);
    let b_exp = b_out | (b_out >> 4);
    (r_exp << 24) | (g_exp << 16) | (b_exp << 8) | a
}

// Compute 8-bit CRC-like checksum using XOR folding (no table).
// Processes four 8-bit lanes of a u32 independently then folds.
fn xor_checksum_u32(data: u32, poly: u32) -> u32 {
    let lane0 = data & 0xFF;
    let lane1 = (data >> 8) & 0xFF;
    let lane2 = (data >> 16) & 0xFF;
    let lane3 = (data >> 24) & 0xFF;
    let step0 = lane0 ^ poly;
    let step1 = lane1 ^ (step0 >> 1);
    let step2 = lane2 ^ (step1 >> 1);
    let step3 = lane3 ^ (step2 >> 1);
    let fold1 = step0 ^ step1;
    let fold2 = fold1 ^ step2;
    let fold3 = fold2 ^ step3;
    fold3 & 0xFF
}

fn main() {}
