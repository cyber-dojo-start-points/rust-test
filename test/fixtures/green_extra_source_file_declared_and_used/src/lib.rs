#![cfg_attr(feature = "strict", deny(warnings))]

mod checksum;

pub fn answer() -> i32 {
    6 * 7
}

pub fn answer_checksum() -> i32 {
    checksum::checksum(answer())
}
