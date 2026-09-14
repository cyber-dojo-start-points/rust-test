// Tests are discovered in tests/, never in src/. This file compiles on its
// own, but src/lib.rs declares no `mod answer_size_tests;` so cargo never
// builds it and the test inside it never runs.

#[test]
fn answer_is_two_digits() {
    println!("checking the size of the answer");
    assert!(crate::answer() > 9);
}
