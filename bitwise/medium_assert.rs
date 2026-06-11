use prusti_contracts::*;

fn reverse_bits_u8_check() {
    // reverse_bits(0b10110001) == 0b10001101
    let x: u8 = 0b1011_0001;
    let b0 = (x >> 7) & 1u8; // 1
    let b1 = (x >> 6) & 1u8; // 0
    let b2 = (x >> 5) & 1u8; // 1
    let b3 = (x >> 4) & 1u8; // 1
    let b4 = (x >> 3) & 1u8; // 0
    let b5 = (x >> 2) & 1u8; // 0
    let b6 = (x >> 1) & 1u8; // 0
    let b7 = x & 1u8;         // 1
    let result = b0 | (b1 << 1) | (b2 << 2) | (b3 << 3)
               | (b4 << 4) | (b5 << 5) | (b6 << 6) | (b7 << 7);
    assert!(result == 0b1000_1101u8);
}

fn bswap_u32_check() {
    let x: u32 = 0x1234_5678;
    let b0 = x & 0xFF;         // 0x78
    let b1 = (x >> 8) & 0xFF;  // 0x56
    let b2 = (x >> 16) & 0xFF; // 0x34
    let b3 = (x >> 24) & 0xFF; // 0x12
    let result = (b0 << 24) | (b1 << 16) | (b2 << 8) | b3;
    assert!(result == 0x7856_3412);
}

fn parity_zero_check() {
    // 0 has even parity.
    let x: u32 = 0;
    let x1 = x ^ (x >> 16);
    let x2 = x1 ^ (x1 >> 8);
    let x3 = x2 ^ (x2 >> 4);
    let x4 = x3 ^ (x3 >> 2);
    let x5 = x4 ^ (x4 >> 1);
    assert!(x5 & 1 == 0);
}

fn parity_three_bits_check() {
    // 0b1011 has 3 set bits -> odd parity = 1.
    let x: u32 = 0b1011;
    let x1 = x ^ (x >> 16); // 0b1011
    let x2 = x1 ^ (x1 >> 8); // 0b1011
    let x3 = x2 ^ (x2 >> 4); // 0b1011
    let x4 = x3 ^ (x3 >> 2); // 0b1011 ^ 0b0010 = 0b1001
    let x5 = x4 ^ (x4 >> 1); // 0b1001 ^ 0b0100 = 0b1101
    let p = x5 & 1;
    assert!(p == 1);
}

fn pack_le_check() {
    let result = (0xAAu32) | (0xBBu32 << 8) | (0xCCu32 << 16) | (0xDDu32 << 24);
    assert!(result == 0xDDCC_BBAAu32);
}

fn main() {}
