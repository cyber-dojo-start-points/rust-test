#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

#[test]
fn answer_is_three_digits() {
    assert!(hiker::answer() > 99);
}

#[test]
fn answer_is_two_digits() {
    assert!(hiker::answer() > 9);
    assert!(hiker::answer() < 100);
}
