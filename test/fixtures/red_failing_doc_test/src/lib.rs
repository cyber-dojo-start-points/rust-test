#![cfg_attr(feature = "strict", deny(warnings))]

/// Gives the answer to life, the universe, and everything.
///
/// ```
/// assert_eq!(43, hiker::answer());
/// ```
pub fn answer() -> i32 {
    6 * 7
}
