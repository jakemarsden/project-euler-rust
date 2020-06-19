//! [Problem 10 - Project Euler](https://projecteuler.net/problem=10)
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! ```rust
//! # use project_euler::p010_summation_of_primes::*;
//! assert_eq!(compute(10), 17)
//! ```
//!
//! Find the sum of all the primes below two million.
//!
//! ```rust
//! # use project_euler::p010_summation_of_primes::*;
//! assert_eq!(compute(2_000_000), 142_913_828_922);
//! ```

use super::util;

pub fn compute(max_exclusive: u32) -> u64 {
    util::primes()
        .take_while(|prime| *prime < max_exclusive as u64)
        .sum()
}
