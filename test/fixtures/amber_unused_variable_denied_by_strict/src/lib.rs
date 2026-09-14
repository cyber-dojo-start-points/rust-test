#![cfg_attr(feature = "strict", deny(warnings))]

// cyber-dojo.sh runs cargo test --features "strict", and the attribute above
// turns every warning into an error, so a variable left behind stops the
// build outright.
pub fn answer() -> i32 {
    let multiplier = 6;
    6 * 7
}
