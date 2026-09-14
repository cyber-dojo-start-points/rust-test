#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

#[test]
fn answer_is_even() {
    assert_eq!(0, hiker::answer() % 2);
}

#[test]
fn life_the_universe_and_everything() {
    assert_eq!(42, hiker::answer());
}
