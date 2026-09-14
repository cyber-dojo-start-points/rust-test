#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

// Cargo builds every tests/*.rs, so the singular name saves nothing: this
// file is a test target of its own and its missing brace stops the build.
#[test]
fn answer_is_three_digits() {
    assert!(hiker::answer() > 99);
