#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

#[test]
fn life_the_universe_and_everything() {
    assert_eq!(42, hiker::answer());
}

#[test]
fn the_answer_ends_in_two() {
    assert_eq!(2, hiker::answer_checksum());
}
