#![cfg_attr(feature = "strict", deny(warnings))]

use hiker;

#[test]
fn life_the_universe_and_everything() {
    let mut total = 0;
    loop {
        total = (total + hiker::answer()) % 1000;
        assert!(total < 1000);
    }
}
