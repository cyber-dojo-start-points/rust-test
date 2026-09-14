#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer() -> i32 {
    6 * multiplier()
}

pub fn answer_digit_sum() -> i32 {
    digit_sum(answer())
}
