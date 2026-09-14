#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

#[test]
fn answer_is_three_digits() {
    assert!(hiker::answer() > 99);
}
