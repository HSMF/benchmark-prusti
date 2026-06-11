use prusti_contracts::*;

// Clamp x to inclusive range [lo, hi].
fn clamp(x: i32, lo: i32, hi: i32) -> i32 {
    if x < lo { lo } else if x > hi { hi } else { x }
}

// Midpoint of two u32 values without overflow.
fn midpoint(a: u32, b: u32) -> u32 {
    if a >= b { b + (a - b) / 2 } else { a + (b - a) / 2 }
}

// Ceiling division of n by d.
#[requires(d > 0)]
fn div_ceil(n: u32, d: u32) -> u32 {
    (n + d - 1) / d
}

// Convert Celsius * 10 to Fahrenheit * 10 (integer, no floats).
fn celsius_to_fahrenheit_x10(c_x10: i32) -> i32 {
    c_x10 * 9 / 5 + 320
}

// Saturating add: a + b capped at cap.
#[requires(a <= cap)]
fn sat_add(a: u32, b: u32, cap: u32) -> u32 {
    if cap - a < b { cap } else { a + b }
}

fn main() {}
