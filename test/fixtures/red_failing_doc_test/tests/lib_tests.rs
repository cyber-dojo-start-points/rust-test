#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

// Every test here passes. The example in the doc comment of src/lib.rs is the
// third target cargo runs, and it is the one that fails.
#[test]
fn life_the_universe_and_everything() {
    assert_eq!(42, hiker::answer());
}
