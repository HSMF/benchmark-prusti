use prusti_contracts::*;

// Swap the low and high nibbles of a byte.
fn swap_nibbles(x: u8) -> u8 {
    let lo = x & 0x0F;
    let hi = x >> 4;
    (lo << 4) | hi
}

// Test whether bit n is set in x.
#[requires(n < 32)]
fn bit_test(x: u32, n: u32) -> bool {
    (x >> n) & 1 == 1
}

// Set bit n in x.
#[requires(n < 32)]
fn bit_set(x: u32, n: u32) -> u32 {
    x | (1u32 << n)
}

// Clear bit n in x.
#[requires(n < 32)]
fn bit_clear(x: u32, n: u32) -> u32 {
    x & !(1u32 << n)
}

// Toggle bit n in x.
#[requires(n < 32)]
fn bit_toggle(x: u32, n: u32) -> u32 {
    x ^ (1u32 << n)
}

// Check whether any bits in mask are set in flags.
fn has_any_flag(flags: u32, mask: u32) -> bool {
    (flags & mask) != 0
}

fn main() {}
