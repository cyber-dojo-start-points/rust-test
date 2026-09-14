#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer() -> i32 {
    eprintln!("answer was called");
    6 * 7
}
