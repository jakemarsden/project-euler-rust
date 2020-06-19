//! [Problem 6 - Project Euler](https://projecteuler.net/problem=6)
//!
//! The sum of the squares of the first ten natural numbers is,
//!
//! ```text
//! 1^2+2^2+...+10^2=385
//! ```
//!
//! The square of the sum of the first ten natural numbers is,
//!
//! ```text
//! (1+2+...+10)^2=55^2=3025
//! ```
//!
//! Hence the difference between the sum of the squares of the first ten natural numbers and the
//! square of the sum is `3025−385=2640`.
//!
//! ```rust
//! # use project_euler::p006_sum_square_difference::*;
//! assert_eq!(compute(10), 2_640);
//! ```
//!
//! Find the difference between the sum of the squares of the first one hundred natural numbers and
//! the square of the sum.
//!
//! ```rust
//! # use project_euler::p006_sum_square_difference::*;
//! assert_eq!(compute(100), 25_164_150);
//! ```

pub fn compute(max: u32) -> u32 {
    let sum_of_squares: u32 = (1..=max).map(|x| x.pow(2)).sum();
    let square_of_sum = nth_triangle_number(max).pow(2);
    difference(sum_of_squares, square_of_sum)
}

/// Return `1+2+3+...+n`
fn nth_triangle_number(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn difference(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
