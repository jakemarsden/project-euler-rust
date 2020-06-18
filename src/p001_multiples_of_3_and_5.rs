//! [Problem 1 - Project Euler](https://projecteuler.net/problem=1)
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
//! The sum of these multiples is 23.
//!
//! ```rust
//! # use project_euler::p001_multiples_of_3_and_5::*;
//! assert_eq!(compute(10), 23)
//! ```
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!
//! ```rust
//! # use project_euler::p001_multiples_of_3_and_5::*;
//! assert_eq!(compute(1_000), 233_168);
//! ```

pub fn compute(max_exclusive: u32) -> u32 {
    let mut sum = 0;
    for i in 1..max_exclusive {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}
