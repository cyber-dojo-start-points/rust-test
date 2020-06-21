#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer() -> i32 {
    6 * 9
}

#[test]
fn life_the_universe_and_everything() {
    assert_eq!(42, answer());
}
