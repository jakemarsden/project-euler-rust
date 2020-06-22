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

pub fn compute(grid_size: u32) -> u64 {
    traverse_grid(grid_size, grid_size)
}

fn traverse_grid(w: u32, h: u32) -> u64 {
    if w == 0 || h == 0 {
        // only one path to the end from here
        1
    } else {
        // at least 2 paths to the end from here
        traverse_grid(w - 1, h) + traverse_grid(w, h - 1)
    }
}
