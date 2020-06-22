//! [Problem 9 - Project Euler](https://projecteuler.net/problem=9)
//!
//! A Pythagorean triplet is a set of three natural numbers, `a < b < c`, for which,
//!
//! ```text
//! a^2 + b^2 = c^2
//! ```
//!
//! For example, `3^2 + 4^2 = 9 + 16 = 25 = 5^2`.
//!
//! There exists exactly one Pythagorean triplet for which `a + b + c = 1000`.
//! Find the product `abc`.
//!
//! ```rust
//! # use project_euler::p009_special_pythagorean_triplet::*;
//! assert_eq!(compute(), 31_875_000)
//! ```

pub fn compute() -> u32 {
    // a + b + c = 1000
    for a in 3..1000 {
        for b in (a + 1)..(1_000 - a) {
            if matches_constraint(a, b) {
                let c = 1_000 - a - b;
                return a * b * c;
            }
        }
    }
    panic!();
}

/// ```text
///   a + b + c = 1000
/// ∴         c = 1000 - (a + b)
///
///   a^2 + b^2 = c^2
/// ∴ a^2 + b^2 = (1000 - (a + b))^2
///             = 1000^2 - 2000(a + b) + (a + b)^2
///             = 1e6 - 2000a - 2000b + a^2 + b^2 + 2ab
///        -1e6 = -2000a - 2000b + 2ab
///         5e5 = 1000a + 1000b - ab
/// ```
fn matches_constraint(a: u32, b: u32) -> bool {
    1_000 * a + 1_000 * b - a * b == 5e5 as u32
}
