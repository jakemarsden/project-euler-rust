use std::iter;

pub fn primes() -> impl Iterator<Item = u64> {
    primes_starting_from(2)
}

pub fn primes_starting_from(first: u64) -> impl Iterator<Item = u64> {
    debug_assert!(first >= 2);
    let mut next_prime = first;
    iter::from_fn(move || {
        let prime = next_prime;
        next_prime += 1 + (prime % 2);
        while !is_prime_assume_odd(next_prime) {
            next_prime += 2;
        }
        Some(prime)
    })
}

fn is_prime_assume_odd(n: u64) -> bool {
    debug_assert!(n > 2);
    debug_assert!(n % 2 != 0);
    let max_divisor = (n as f64).sqrt().ceil() as u64;
    !(3..=max_divisor).step_by(2).any(|divisor| n % divisor == 0)
}

pub fn add_numeric_strs(addend: &str, accumulator: &mut String) {
    if addend.len() > accumulator.capacity() {
        // Since we need to grow anyway, add capacity for the carry digit in case it's needed
        accumulator.reserve_exact(addend.len() - accumulator.len() + 1);
    }
    if addend.len() > accumulator.len() {
        let placeholder = "0".repeat(addend.len() - accumulator.len());
        accumulator.insert_str(0, placeholder.as_str());
    }

    let add_bytes = addend.as_bytes();
    let mut add_idx = add_bytes.len();
    let mut carry = false;
    let acc_bytes = unsafe { accumulator.as_mut_vec() };
    for acc_idx in (0..acc_bytes.len()).rev() {
        let mut acc = acc_bytes.get_mut(acc_idx).unwrap();
        let add = if add_idx > 0 {
            add_idx -= 1;
            add_bytes[add_idx]
        } else {
            b'0'
        };
        add_numeric_digits(add, &mut acc, &mut carry);
    }
    if carry {
        acc_bytes.insert(0, b'1');
    }
}

fn add_numeric_digits(addend: u8, accumulator: &mut u8, carry: &mut bool) {
    let acc = from_numeric_char(*accumulator);
    let add = from_numeric_char(addend);
    let out = acc + add + *carry as u8;

    *accumulator = to_numeric_char(out % 10);
    *carry = out / 10 != 0;
}

#[inline]
pub fn from_numeric_char(digit: u8) -> u8 {
    debug_assert!(digit >= b'0' && digit <= b'9');
    digit - b'0'
}

#[inline]
pub fn to_numeric_char(num: u8) -> u8 {
    debug_assert!(num < 10);
    num + b'0'
}
