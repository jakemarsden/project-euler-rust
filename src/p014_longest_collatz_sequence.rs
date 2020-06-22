//! [Problem 14 - Project Euler](https://projecteuler.net/problem=14)
//!
//! The following iterative sequence is defined for the set of positive integers:
//!
//! ```text
//! n → n/2 (n is even)
//! n → 3n + 1 (n is odd)
//! ```
//!
//! Using the rule above and starting with 13, we generate the following sequence:
//!
//! ```text
//! 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//! ```
//!
//! It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
//! Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers
//! finish at 1.
//!
//! Which starting number, under one million, produces the longest chain?
//!
//! ```rust
//! # use project_euler::p014_longest_collatz_sequence::*;
//! assert_eq!(compute(1_000_000), 837_799);
//! ```
//!
//! NOTE: Once the chain starts the terms are allowed to go above one million.

pub fn compute(max_seed_exclusive: u32) -> u32 {
    let mut longest_chain_len = 1;
    let mut longest_chain_seed = 1;
    for seed in 2..max_seed_exclusive {
        let chain_len = collatz_len(seed);
        if chain_len > longest_chain_len {
            longest_chain_len = chain_len;
            longest_chain_seed = seed;
        }
    }
    longest_chain_seed
}

fn collatz_len(seed: u32) -> u32 {
    let mut n = seed as u64;
    let mut len = 1;
    loop {
        while n % 2 == 0 {
            n /= 2;
            len += 1;
            if n == 1 {
                return len;
            }
        }
        n = 3 * n + 1;
        n /= 2;
        len += 2;
    }
}
