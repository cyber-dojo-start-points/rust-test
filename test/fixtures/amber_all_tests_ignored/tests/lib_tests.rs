#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

// The learner parked the only test that would have caught 6 * 9 and never
// came back to it.
#[test]
#[ignore]
fn life_the_universe_and_everything() {
    assert_eq!(42, hiker::answer());
}
