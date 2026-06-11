use prusti_contracts::*;

fn swap_nibbles_check() {
    // 0xAB -> 0xBA
    let x: u8 = 0xAB;
    let lo = x & 0x0F;  // 0x0B
    let hi = x >> 4;     // 0x0A
    let result = (lo << 4) | hi;
    assert!(result == 0xBAu8);
}

fn bit_test_check() {
    let x: u32 = 0b1010_0110;
    let b1 = (x >> 1) & 1; // set
    let b2 = (x >> 2) & 1; // set
    let b3 = (x >> 3) & 1; // clear
    assert!(b1 == 1);
    assert!(b2 == 1);
    assert!(b3 == 0);
}

fn bit_set_check() {
    let x: u32 = 0b0101;
    let result = x | (1u32 << 1);
    assert!(result == 0b0111);
}

fn bit_clear_check() {
    let x: u32 = 0b1111;
    let result = x & !(1u32 << 2);
    assert!(result == 0b1011);
}

fn bit_toggle_check() {
    let x: u32 = 0b1010;
    let r1 = x ^ (1u32 << 1); // toggle bit 1: 0b1000
    let r2 = r1 ^ (1u32 << 1); // toggle back: 0b1010
    assert!(r1 == 0b1000);
    assert!(r2 == 0b1010);
}

fn has_any_flag_check() {
    let flags: u32 = 0b0001_0100;
    let present = (flags & 0b0000_0100) != 0;
    let absent = (flags & 0b0000_0010) != 0;
    assert!(present);
    assert!(!absent);
}

fn main() {}
