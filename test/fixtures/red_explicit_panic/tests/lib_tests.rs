#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

// A panic that is not an assertion is still a test failure, so this reaches
// red exactly as a broken assert_eq! does.
#[test]
fn life_the_universe_and_everything() {
    assert_eq!(42, hiker::answer());
}
