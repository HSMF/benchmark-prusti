use prusti_contracts::*;

fn reverse_bits_u16_palindrome_check() {
    // 0x8001 = 0b1000_0000_0000_0001 is a bit-palindrome.
    let x: u16 = 0x8001;
    let b0  = (x >> 15) & 1u16; // 1
    let b1  = (x >> 14) & 1u16; // 0
    let b2  = (x >> 13) & 1u16; // 0
    let b3  = (x >> 12) & 1u16; // 0
    let b4  = (x >> 11) & 1u16; // 0
    let b5  = (x >> 10) & 1u16; // 0
    let b6  = (x >> 9)  & 1u16; // 0
    let b7  = (x >> 8)  & 1u16; // 0
    let b8  = (x >> 7)  & 1u16; // 0
    let b9  = (x >> 6)  & 1u16; // 0
    let b10 = (x >> 5)  & 1u16; // 0
    let b11 = (x >> 4)  & 1u16; // 0
    let b12 = (x >> 3)  & 1u16; // 0
    let b13 = (x >> 2)  & 1u16; // 0
    let b14 = (x >> 1)  & 1u16; // 0
    let b15 = x & 1u16;          // 1
    let result = b0 | (b1 << 1)  | (b2 << 2)  | (b3 << 3)
               | (b4 << 4)  | (b5 << 5)  | (b6 << 6)  | (b7 << 7)
               | (b8 << 8)  | (b9 << 9)  | (b10 << 10) | (b11 << 11)
               | (b12 << 12) | (b13 << 13) | (b14 << 14) | (b15 << 15);
    assert!(result == 0x8001u16);
}

fn reverse_bits_u16_known_check() {
    // reverse(0x0001) == 0x8000
    let x: u16 = 0x0001;
    let b0  = (x >> 15) & 1u16; // 0
    let b1  = (x >> 14) & 1u16; // 0
    let b2  = (x >> 13) & 1u16; // 0
    let b3  = (x >> 12) & 1u16; // 0
    let b4  = (x >> 11) & 1u16; // 0
    let b5  = (x >> 10) & 1u16; // 0
    let b6  = (x >> 9)  & 1u16; // 0
    let b7  = (x >> 8)  & 1u16; // 0
    let b8  = (x >> 7)  & 1u16; // 0
    let b9  = (x >> 6)  & 1u16; // 0
    let b10 = (x >> 5)  & 1u16; // 0
    let b11 = (x >> 4)  & 1u16; // 0
    let b12 = (x >> 3)  & 1u16; // 0
    let b13 = (x >> 2)  & 1u16; // 0
    let b14 = (x >> 1)  & 1u16; // 0
    let b15 = x & 1u16;          // 1
    let result = b0 | (b1 << 1)  | (b2 << 2)  | (b3 << 3)
               | (b4 << 4)  | (b5 << 5)  | (b6 << 6)  | (b7 << 7)
               | (b8 << 8)  | (b9 << 9)  | (b10 << 10) | (b11 << 11)
               | (b12 << 12) | (b13 << 13) | (b14 << 14) | (b15 << 15);
    assert!(result == 0x8000u16);
}

fn rgba_mask_identity_check() {
    // Full mask (0xFF) leaves channels unchanged; expansion: x | (x >> 4).
    let packed: u32 = 0xFF00_FF00;
    let r = (packed >> 24) & 0xFF; // 0xFF
    let g = (packed >> 16) & 0xFF; // 0x00
    let b = (packed >> 8) & 0xFF;  // 0xFF
    let a = packed & 0xFF;          // 0x00
    let r_out = r & 0xFF;  // 0xFF
    let g_out = g & 0xFF;  // 0x00
    let b_out = b & 0xFF;  // 0xFF
    let r_exp = r_out | (r_out >> 4); // 0xFF | 0x0F = 0xFF
    let g_exp = g_out | (g_out >> 4); // 0x00
    let b_exp = b_out | (b_out >> 4); // 0xFF
    let result = (r_exp << 24) | (g_exp << 16) | (b_exp << 8) | a;
    assert!(result == 0xFF00_FF00u32);
}

fn xor_checksum_idempotent_check() {
    // XOR with zero poly on zero data gives 0.
    let data: u32 = 0;
    let poly: u32 = 0;
    let lane0 = data & 0xFF;          // 0
    let lane1 = (data >> 8) & 0xFF;   // 0
    let lane2 = (data >> 16) & 0xFF;  // 0
    let lane3 = (data >> 24) & 0xFF;  // 0
    let step0 = lane0 ^ poly;         // 0
    let step1 = lane1 ^ (step0 >> 1); // 0
    let step2 = lane2 ^ (step1 >> 1); // 0
    let step3 = lane3 ^ (step2 >> 1); // 0
    let fold1 = step0 ^ step1;        // 0
    let fold2 = fold1 ^ step2;        // 0
    let fold3 = fold2 ^ step3;        // 0
    let result = fold3 & 0xFF;
    assert!(result == 0);
}

fn main() {}
