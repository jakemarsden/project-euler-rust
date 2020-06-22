//! [Problem 15 - Project Euler](https://projecteuler.net/problem=15)
//!
//! Starting in the top left corner of a 2×2 grid, and only being able to move to the right and
//! down, there are exactly 6 routes to the bottom right corner.
//!
//! ![](https://projecteuler.net/project/images/p015.png)
//!
//! ```rust
//! # use project_euler::p015_lattice_paths::*;
//! assert_eq!(compute(2), 6);
//! ```
//!
//! How many such routes are there through a 20×20 grid?
//!
//! ```rust
//! # use project_euler::p015_lattice_paths::*;
//! assert_eq!(compute(20), 137_846_528_820);
//! ```

/// ```text
/// path_count =                 (2n)! / (n!)^2
///            = (n!)(n+1)(n+2)...(2n) / (n!)(n!)
///            =     (n+1)(n+2)...(2n) / n!
/// ```
pub fn compute(grid_size: u32) -> u64 {
    let n = grid_size as u128;
    let dividend = (n + 1..=2 * n).product::<u128>();
    let divisor = (1..=n).product::<u128>();
    (dividend / divisor) as u64
}
