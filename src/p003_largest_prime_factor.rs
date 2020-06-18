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

pub fn compute(n: u64) -> u64 {
    if n % 2 == 0 {
        return compute(n / 2);
    }
    let max_factor = (n as f64).sqrt().ceil() as u64;
    for factor1 in (3..max_factor).step_by(2) {
        if n % factor1 != 0 {
            continue;
        }
        let factor2 = n / factor1;
        return u64::max(compute(factor1), compute(factor2));
    }
    // no factors, must be prime!
    return n;
}
