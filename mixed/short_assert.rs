use prusti_contracts::*;

fn align_up_check() {
    // align_up(13, 8) == 16
    let x = 13u32;
    let align = 8u32;
    let mask = align - 1; // 7
    let result = (x + mask) & !mask; // (20) & 0xFFFFFFF8 = 16
    assert!(result == 16);
}

fn align_up_already_aligned_check() {
    // align_up(16, 8) == 16
    let x = 16u32;
    let mask = 7u32;
    let result = (x + mask) & !mask;
    assert!(result == 16);
}

fn nibble_to_byte_max_check() {
    // max nibble 15 -> 255
    let n: u8 = 15u8 & 0x0F;
    let result = n * 17;
    assert!(result == 255u8);
}

fn nibble_to_byte_zero_check() {
    let n: u8 = 0u8 & 0x0F;
    let result = n * 17;
    assert!(result == 0u8);
}

fn pack_rgb565_red_check() {
    // Pure max-red: R=31, G=0, B=0 -> 0xF800
    let result = ((31u32 & 0x1F) << 11) | ((0u32 & 0x3F) << 5) | (0u32 & 0x1F);
    assert!(result == 0xF800);
}

fn pack_rgb565_blue_check() {
    // Pure max-blue: R=0, G=0, B=31 -> 0x001F
    let result = ((0u32 & 0x1F) << 11) | ((0u32 & 0x3F) << 5) | (31u32 & 0x1F);
    assert!(result == 0x001F);
}

fn quantize_5bit_max_check() {
    // 255 >> 3 = 31; (31 << 3) | (31 >> 2) = 248 | 7 = 255
    let x: u8 = 255;
    let q = x >> 3;
    let result = (q << 3) | (q >> 2);
    assert!(result == 255u8);
}

fn main() {}
