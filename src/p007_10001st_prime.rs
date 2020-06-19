//! [Problem 7 - Project Euler](https://projecteuler.net/problem=7)
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is
//! 13.
//!
//! ```rust
//! # use project_euler::p007_10001st_prime::*;
//! assert_eq!(compute(6), 13);
//! ```
//!
//! What is the 10 001st prime number?
//!
//! ```rust
//! # use project_euler::p007_10001st_prime::*;
//! assert_eq!(compute(10_001), 104_743);
//! ```

use super::util;

pub fn compute(n: u32) -> u32 {
    util::primes().nth(n as usize - 1).unwrap() as u32
}
