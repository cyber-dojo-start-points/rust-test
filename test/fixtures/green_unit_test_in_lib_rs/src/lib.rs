#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer() -> i32 {
    6 * 7
}

// A #[cfg(test)] module inside src/lib.rs is the lib target's own test, so it
// runs in the first of the three "test result:" lines rather than the second.
#[cfg(test)]
mod tests {
    #[test]
    fn answer_is_two_digits() {
        assert!(super::answer() > 9);
        assert!(super::answer() < 100);
    }
}
