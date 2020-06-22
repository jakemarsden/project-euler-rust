//! [Problem 18 - Project Euler](https://projecteuler.net/problem=18)
//!
//! By starting at the top of the triangle below and moving to adjacent numbers on the row below,
//! the maximum total from top to bottom is 23.
//!
//! ```text
//!    3
//!   7 4
//!  2 4 6
//! 8 5 9 3
//! ```
//!
//! That is, `3 + 7 + 4 + 9 = 23`.
//!
//! ```rust
//! # use project_euler::p018_maximum_path_sum_i::*;
//!
//! # #[rustfmt::skip]
//! # static TREE: [&[u8]; 4] = [
//! #     &[   3      ],
//! #     &[  7, 4    ],
//! #     &[ 2, 4, 6  ],
//! #     &[8, 5, 9, 3],
//! # ];
//! assert_eq!(compute(&TREE), 23);
//! ```
//!
//! Find the maximum total from top to bottom of the triangle below:
//!
//! ```text
//!               75
//!              95 64
//!             17 47 82
//!            18 35 87 10
//!           20 04 82 47 65
//!          19 01 23 75 03 34
//!         88 02 77 73 07 63 67
//!        99, 65 04 28 06 16 70 92
//!       41 41 26 56 83 40 80 70 33
//!      41 48 72 33 47 32 37 16 94 29
//!     53 71 44 65 25 43 91 52 97 51 14
//!    70 11 33 28 77 73 17 78 39 68 17 57
//!   91 71 52 38 17 14 91 43 58 50 27 29 48
//!  63 66 04 68 89 53 67 30 73 16 69 87 40 31
//! 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
//! ```
//!
//! ```rust
//! # use project_euler::p018_maximum_path_sum_i::*;
//!
//! # #[rustfmt::skip]
//! # static TREE: [&[u8]; 15] = [
//! #     &[                            75                            ],
//! #     &[                          95, 64                          ],
//! #     &[                        17, 47, 82                        ],
//! #     &[                      18, 35, 87, 10                      ],
//! #     &[                    20, 04, 82, 47, 65                    ],
//! #     &[                  19, 01, 23, 75, 03, 34                  ],
//! #     &[                88, 02, 77, 73, 07, 63, 67                ],
//! #     &[              99, 65, 04, 28, 06, 16, 70, 92              ],
//! #     &[            41, 41, 26, 56, 83, 40, 80, 70, 33            ],
//! #     &[          41, 48, 72, 33, 47, 32, 37, 16, 94, 29          ],
//! #     &[        53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14        ],
//! #     &[      70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57      ],
//! #     &[    91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48    ],
//! #     &[  63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31  ],
//! #     &[04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
//! # ];
//! assert_eq!(compute(&TREE), 1_074);
//! ```
//!
//! NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every
//! route. However, [Problem 67](https://projecteuler.net/problem=67), is the same challenge with a
//! triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever
//! method! ;o)

use std::cmp;
use std::fmt;
use Tree::*;

#[derive(fmt::Debug)]
enum Tree<T> {
    Branch(T, Box<Tree<T>>, Box<Tree<T>>),
    Leaf(T),
}

impl<T> Tree<T> {
    fn branch(value: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Branch(value, Box::new(left), Box::new(right))
    }

    fn leaf(value: T) -> Tree<T> {
        Leaf(value)
    }
}

pub fn compute(flat_tree: &[&[u8]]) -> u32 {
    let tree = unpack_tree(flat_tree, 0, 0);
    max_path_len(&tree)
}

fn max_path_len(node: &Tree<u8>) -> u32 {
    match node {
        Leaf(n) => *n as u32,
        Branch(n, left, right) => *n as u32 + cmp::max(max_path_len(left), max_path_len(right)),
    }
}

fn unpack_tree<T: Copy>(values: &[&[T]], offset: usize, depth: usize) -> Tree<T> {
    if depth + 1 < values.len() {
        Tree::branch(
            values[depth][offset],
            unpack_tree(values, offset, depth + 1),
            unpack_tree(values, offset + 1, depth + 1),
        )
    } else {
        // No children, must be a leaf
        Tree::leaf(values[depth][offset])
    }
}
