//! [Problem 17 - Project Euler](https://projecteuler.net/problem=17)
//!
//! If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are
//! `3 + 3 + 5 + 4 + 4 = 19` letters used in total.
//!
//! ```rust
//! # use project_euler::p017_number_letter_counts::*;
//! assert_eq!(compute(5), 19);
//! ```
//!
//! If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
//! letters would be used?
//!
//! ```rust
//! # use project_euler::p017_number_letter_counts::*;
//! assert_eq!(compute(1_000), 21_124);
//! ```
//!
//! NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23
//! letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out
//! numbers is in compliance with British usage.

use std::fmt;
use std::ops;

#[derive(Clone, Copy, fmt::Debug)]
struct StringLen {
    len: usize,
}

impl StringLen {
    pub fn empty() -> StringLen {
        StringLen { len: 0 }
    }

    pub fn from(str: &str) -> StringLen {
        StringLen { len: str.len() }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl ops::Add<StringLen> for StringLen {
    type Output = StringLen;

    fn add(self, rhs: StringLen) -> Self::Output {
        let len = self.len() + rhs.len();
        StringLen { len }
    }
}

impl ops::Add<&str> for StringLen {
    type Output = StringLen;

    fn add(self, rhs: &str) -> Self::Output {
        self + StringLen::from(rhs)
    }
}

impl ops::AddAssign<StringLen> for StringLen {
    fn add_assign(&mut self, rhs: StringLen) {
        *self = *self + rhs;
    }
}

impl ops::AddAssign<&str> for StringLen {
    fn add_assign(&mut self, rhs: &str) {
        *self = *self + rhs;
    }
}

pub fn compute(max: u32) -> usize {
    (1..=max)
        .map(number_word_len)
        .map(|word_len| word_len.len())
        .sum()
}

fn number_word_len(mut n: u32) -> StringLen {
    debug_assert!(n < 1_000_000);
    match n {
        1 => StringLen::from("one"),
        2 => StringLen::from("two"),
        3 => StringLen::from("three"),
        4 => StringLen::from("four"),
        5 => StringLen::from("five"),
        6 => StringLen::from("six"),
        7 => StringLen::from("seven"),
        8 => StringLen::from("eight"),
        9 => StringLen::from("nine"),

        10 => StringLen::from("ten"),
        11 => StringLen::from("eleven"),
        12 => StringLen::from("twelve"),
        13 => StringLen::from("thirteen"),
        15 => StringLen::from("fifteen"),
        18 => StringLen::from("eighteen"),
        14 | 16 | 17 | 19 => number_word_len(n % 10) + "teen",

        20 => StringLen::from("twenty"),
        30 => StringLen::from("thirty"),
        40 => StringLen::from("forty"),
        50 => StringLen::from("fifty"),
        80 => StringLen::from("eighty"),
        60 | 70 | 90 => number_word_len(n / 10) + "ty",

        _ => {
            let thousands = n / 1_000;
            if thousands != 0 {
                n %= 1_000 * thousands;
            }
            let hundreds = n / 100;
            if hundreds != 0 {
                n %= 100 * hundreds;
            }
            let teens = n;
            let tens = n / 10;
            if tens != 0 {
                n %= 10 * tens;
            }
            let ones = n;

            let mut val = StringLen::empty();
            if thousands != 0 {
                val += number_word_len(thousands);
                val += "thousand";
                if hundreds != 0 && teens != 0 {
                    val += "and";
                }
            }
            if hundreds != 0 {
                val += number_word_len(hundreds);
                val += "hundred";
                if teens != 0 {
                    val += "and";
                }
            }
            if teens != 0 && teens < 20 {
                val += number_word_len(teens);
            }
            if teens >= 20 {
                if tens != 0 {
                    val += number_word_len(10 * tens);
                }
                if ones != 0 {
                    val += number_word_len(ones);
                }
            }
            val
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn three_hundred_and_forty_two() {
        assert_eq!(super::number_word_len(342).len(), 23);
    }

    #[test]
    fn one_hundred_and_fifteen() {
        assert_eq!(super::number_word_len(115).len(), 20);
    }
}
