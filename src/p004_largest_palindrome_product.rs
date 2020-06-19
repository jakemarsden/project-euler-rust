//! [Problem 4 - Project Euler](https://projecteuler.net/problem=4)
//!
//! A palindromic number reads the same both ways. The largest palindrome made from the product of
//! two 2-digit numbers is 9009 = 91 × 99.
//!
//! ```rust
//! # use project_euler::p004_largest_palindrome_product::*;
//! assert_eq!(compute(2), 90_09);
//! ```
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! ```rust
//! # use project_euler::p004_largest_palindrome_product::*;
//! assert_eq!(compute(3), 906_609);
//! ```

pub fn compute(digits: u32) -> u32 {
    debug_assert_ne!(digits, 0);
    let min = 10_u32.pow(digits - 1) as u32;
    let max = 10_u32.pow(digits) as u32 - 1;

    let mut max_product = 0;
    for lhs in min..=max {
        for rhs in lhs..=max {
            let product = lhs * rhs;
            if product > max_product && is_palindrome(product) {
                max_product = product;
            }
        }
    }
    max_product
}

fn is_palindrome(n: u32) -> bool {
    let str = n.to_string();
    let digits = str.as_bytes();

    let mut front_idx = 0;
    let mut back_idx = digits.len() - 1;
    while front_idx < back_idx {
        if digits[front_idx] != digits[back_idx] {
            return false;
        }
        front_idx += 1;
        back_idx -= 1;
    }
    true
}
