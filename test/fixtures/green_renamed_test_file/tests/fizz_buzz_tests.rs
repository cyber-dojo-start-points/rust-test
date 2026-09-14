#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

// tests/lib_tests.rs is gone and this file took its place. Cargo makes a test
// target out of whatever tests/*.rs it finds, so the name is the learner's to
// choose.
#[test]
fn life_the_universe_and_everything() {
    assert_eq!(42, hiker::answer());
}
