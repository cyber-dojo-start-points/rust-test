#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer() -> i32 {
    six() * 7
}

// six() and seven() call each other, so answer() never returns. The recursion
// goes through two functions because a function that calls itself directly is
// caught by the unconditional_recursion lint, which --features "strict" turns
// into a build error.
fn six() -> i32 {
    seven() - 1
}

fn seven() -> i32 {
    six() + 1
}

#[cfg(test)]
mod tests {
    // The lib target is the first cargo runs. The overflow aborts its process
    // before it prints a "test result:" line, and cargo runs nothing after it,
    // so no summary line reaches the lambda at all.
    #[test]
    fn life_the_universe_and_everything() {
        assert_eq!(42, super::answer());
    }
}
