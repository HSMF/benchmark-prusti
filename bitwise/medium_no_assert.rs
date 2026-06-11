use prusti_contracts::*;

// Reverse all 8 bits of a byte (fully unrolled).
fn reverse_bits_u8(x: u8) -> u8 {
    let b0 = (x >> 7) & 1u8;
    let b1 = (x >> 6) & 1u8;
    let b2 = (x >> 5) & 1u8;
    let b3 = (x >> 4) & 1u8;
    let b4 = (x >> 3) & 1u8;
    let b5 = (x >> 2) & 1u8;
    let b6 = (x >> 1) & 1u8;
    let b7 = x & 1u8;
    b0 | (b1 << 1) | (b2 << 2) | (b3 << 3)
       | (b4 << 4) | (b5 << 5) | (b6 << 6) | (b7 << 7)
}

// Reverse byte order of a u32 (endian swap).
fn bswap_u32(x: u32) -> u32 {
    let b0 = x & 0xFF;
    let b1 = (x >> 8) & 0xFF;
    let b2 = (x >> 16) & 0xFF;
    let b3 = (x >> 24) & 0xFF;
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}

// Compute bitwise parity of a u32 (1 if odd number of set bits).
fn parity_u32(x: u32) -> u32 {
    let x1 = x ^ (x >> 16);
    let x2 = x1 ^ (x1 >> 8);
    let x3 = x2 ^ (x2 >> 4);
    let x4 = x3 ^ (x3 >> 2);
    let x5 = x4 ^ (x4 >> 1);
    x5 & 1
}

// Pack four u8 values into a u32 (little-endian: a in bits [7:0]).
fn pack_u8x4_le(a: u32, b: u32, c: u32, d: u32) -> u32 {
    (a & 0xFF) | ((b & 0xFF) << 8) | ((c & 0xFF) << 16) | ((d & 0xFF) << 24)
}

fn main() {}
