//! [Problem 19 - Project Euler](https://projecteuler.net/problem=19)
//!
//! You are given the following information, but you may prefer to do some research for yourself.
//!
//! - 1 Jan 1900 was a Monday.<br>
//! - Thirty days has September,<br>
//!   April, June and November.<br>
//!   All the rest have thirty-one,<br>
//!   Saving February alone,<br>
//!   Which has twenty-eight, rain or shine.<br>
//!   And on leap years, twenty-nine.
//! - A leap year occurs on any year evenly divisible by 4, but not on a century unless it is
//!   divisible by 400.
//!
//! How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31
//! Dec 2000)?
//!
//! ```rust
//! # use project_euler::p019_counting_sundays::*;
//! assert_eq!(compute(), 171);
//! ```

use std::fmt;
use std::iter;

#[derive(Copy, Clone, fmt::Debug)]
struct Date {
    year: u32,
    month: u8,
    day_of_month: u8,
    day_of_week: u8,
}

impl Date {
    fn new(year: u32, month: u8, day_of_month: u8, day_of_week: u8) -> Date {
        debug_assert!(month < 12);
        debug_assert!(day_of_month < Date::month_len(year, month));
        debug_assert!(day_of_week < 7);
        Date {
            year,
            month,
            day_of_month,
            day_of_week,
        }
    }

    fn month_len(year: u32, month: u8) -> u8 {
        debug_assert!(month < 12);
        match month {
            1 => {
                if Date::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            3 | 5 | 8 | 10 => 30,
            _ => 31,
        }
    }

    fn is_leap_year(year: u32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    fn next(&self) -> Date {
        let mut year = self.year;
        let mut month = self.month;
        let mut dom = self.day_of_month;
        let mut dow = self.day_of_week;

        dow += 1;
        dow %= 7;

        dom += 1;
        if dom >= Date::month_len(year, month) {
            dom = 0;
            month += 1;
            if month >= 12 {
                month = 0;
                year += 1;
            }
        }
        Date::new(year, month, dom, dow)
    }
}

pub fn compute() -> u32 {
    //! How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31
    //! Dec 2000)?
    let start_date = Date::new(1900, 0, 0, 0);
    date_iter(&start_date)
        .skip_while(|date| date.year < 1901)
        .take_while(|date| date.year <= 2000)
        .filter(|date| date.day_of_month == 0)
        .filter(|date| date.day_of_week == 6)
        .count() as u32
}

fn date_iter(start: &Date) -> impl Iterator<Item = Date> {
    iter::successors(Some(*start), |prev| Some(prev.next()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn today() {
        let start_date = super::Date::new(1900, 0, 0, 0);
        let today = super::date_iter(&start_date)
            .take_while(|date| date.year < 2020 || date.month < 5 || date.day_of_month <= 22)
            .last();
        assert!(today.is_some());
        assert_eq!(today.unwrap().year, 2020); // 2020
        assert_eq!(today.unwrap().month, 5); // June
        assert_eq!(today.unwrap().day_of_month, 22); // 23rd
        assert_eq!(today.unwrap().day_of_week, 1); // Tuesday
    }
}
