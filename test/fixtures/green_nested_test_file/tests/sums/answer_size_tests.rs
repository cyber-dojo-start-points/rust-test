#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

// Cargo makes a test target out of each tests/*.rs, so a file one level down
// is never built and this test never runs.
#[test]
fn answer_is_two_digits() {
    println!("checking the size of the answer");
    assert!(hiker::answer() > 9);
    assert!(hiker::answer() < 100);
}
