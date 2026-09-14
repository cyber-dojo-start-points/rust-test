#![cfg_attr(feature = "strict", deny(warnings))]

pub fn answer() -> i32 {
    6 * 7
}

#[cfg(test)]
mod tests {
    // The learner put a print inside a loop to see what was happening. The
    // lib target is the first cargo runs, so this flood fills the head of
    // stdout and every "test result:" line lands past the 50K the runner
    // keeps.
    #[test]
    fn life_the_universe_and_everything() {
        let mut total = 0;
        for i in 0..5000 {
            println!("debug: i is {}, total is {}", i, total);
            total += 1;
        }
        assert_eq!(42, super::answer());
    }
}
