use prusti_contracts::*;

fn clamp_upper_check() {
    let x = 150i32;
    let result = if x < 0 { 0 } else if x > 100 { 100 } else { x };
    assert!(result == 100);
}

fn clamp_lower_check() {
    let x = -5i32;
    let result = if x < 0 { 0 } else if x > 100 { 100 } else { x };
    assert!(result == 0);
}

fn midpoint_check() {
    let a = 10u32;
    let b = 20u32;
    let m = if a >= b { b + (a - b) / 2 } else { a + (b - a) / 2 };
    assert!(m == 15);
}

fn div_ceil_check() {
    let n = 10u32;
    let d = 3u32;
    let result = (n + d - 1) / d;
    assert!(result == 4);
}

fn fahrenheit_check() {
    // 100 C * 10 -> 212 F * 10
    let c_x10 = 1000i32;
    let f_x10 = c_x10 * 9 / 5 + 320;
    assert!(f_x10 == 2120);
}

fn sat_add_overflow_check() {
    let a = 200u32;
    let b = 100u32;
    let cap = 255u32;
    let result = if cap - a < b { cap } else { a + b };
    assert!(result == 255);
}

fn sat_add_no_overflow_check() {
    let a = 100u32;
    let b = 50u32;
    let cap = 255u32;
    let result = if cap - a < b { cap } else { a + b };
    assert!(result == 150);
}

fn main() {}
