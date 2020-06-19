//! [Problem 5 - Project Euler](https://projecteuler.net/problem=5)
//!
//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
//! remainder.
//!
//! ```rust
//! # use project_euler::p005_smallest_multiple::*;
//! assert_eq!(compute(10), 2_520);
//! ```
//!
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to
//! 20?
//!
//! ```rust
//! # use project_euler::p005_smallest_multiple::*;
//! assert_eq!(compute(20), 232_792_560);
//! ```

pub fn compute(max_divisor: u32) -> u32 {
    let min_divisor = (max_divisor / 2) + 1;
    (max_divisor..)
        .step_by(max_divisor as usize)
        .find(|n| {
            (min_divisor..max_divisor)
                .rev()
                .all(|divisor| n % divisor == 0)
        })
        .unwrap()
}
