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
