//! [Problem 3 - Project Euler](https://projecteuler.net/problem=3)
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! ```rust
//! # use project_euler::p003_largest_prime_factor::*;
//! assert_eq!(compute(13_195), 29);
//! ```
//!
//! What is the largest prime factor of the number 600851475143 ?
//!
//! ```rust
//! # use project_euler::p003_largest_prime_factor::*;
//! assert_eq!(compute(600_851_475_143), 6_857);
//! ```

use std::cmp;

pub fn compute(n: u64) -> u64 {
    if n % 2 == 0 {
        return if n == 2 { 2 } else { compute(n / 2) };
    }
    let max_factor = (n as f64).sqrt().ceil() as u64;
    let factor = (3..max_factor).step_by(2).find(|x| n % *x == 0);
    match factor {
        Some(x) => cmp::max(compute(x), compute(n / x)),
        None => n,
    }
}
