#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

#[test]
fn life_the_universe_and_everything() {
    assert_eq!(42, hiker::answer());
}

#[test]
fn digits_of_the_answer_sum_to_six() {
    assert_eq!(6, hiker::answer_digit_sum());
}
