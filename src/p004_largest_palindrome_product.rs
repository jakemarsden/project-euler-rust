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

    let mut min = 1;
    for _ in 0..digits - 1 {
        min *= 10;
    }
    let max = min * 10 - 1;

    let mut max_palindromic_product = 1;
    for lhs in min..=max {
        for rhs in lhs..=max {
            let product = lhs * rhs;
            if product > max_palindromic_product && is_palindromic(product) {
                max_palindromic_product = product;
            }
        }
    }
    max_palindromic_product
}

fn is_palindromic(n: u32) -> bool {
    if n < 10 {
        return true;
    }
    if n <= 100 {
        return n % 11 == 0;
    }

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
