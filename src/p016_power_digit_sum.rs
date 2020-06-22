//! [Problem 16 - Project Euler](https://projecteuler.net/problem=16)
//!
//! `2^15 = 32768` and the sum of its digits is `3 + 2 + 7 + 6 + 8 = 26`.
//!
//! ```rust
//! # use project_euler::p016_power_digit_sum::*;
//! assert_eq!(compute(15), 26)
//! ```
//!
//! What is the sum of the digits of the number `2^1000`?
//!
//! ```rust
//! # use project_euler::p016_power_digit_sum::*;
//! assert_eq!(compute(1_000), 1_366);
//! ```

use super::util;

pub fn compute(exponent: u32) -> u32 {
    let n = power_of_2_numeric_str(&exponent);
    n.as_bytes().iter()
        .map(|ch| util::from_numeric_char(*ch) as u32)
        .sum()
}

fn power_of_2_numeric_str(exponent: &u32) -> String {
    let mut accumulator = String::from("1");
    for _ in 0..*exponent {
        util::add_numeric_strs(accumulator.clone().as_str(), &mut accumulator);
    }
    accumulator
}
