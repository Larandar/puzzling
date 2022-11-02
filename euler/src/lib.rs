/// All the useful series, each function return an Interator
pub mod series {
    pub use crate::primes::{factors, primes};

    /// Return a new Interator of fibonacci numbers
    pub fn fibonacci() -> impl Iterator<Item = u128> {
        let mut fib: [u128; 2] = [0, 1];
        std::iter::from_fn(move || {
            fib.swap(0, 1);
            let next = fib.iter().sum();
            Some(std::mem::replace(&mut fib[1], next))
        })
    }
}

/// Everything related to primicity
pub mod primes {
    use rayon::prelude::*;

    /// Test if a number is prime using Miller-Rabin test
    pub fn is_prime(number: u128) -> bool {
        return if number == 2 || number == 3 {
            true
        } else if number < 2 || number % 2 == 0 {
            false
        } else if number < u64::MAX as u128 {
            miller_rabin_test(number as u128)
        } else {
            unimplemented!("is_prime for number greater than u64::MAX")
        };
    }

    /// Infinite (until it panic) iterator that yield prime after prime
    pub fn primes() -> impl Iterator<Item = u128> {
        (2_u128..)
            .filter(|&n| (n == 2) || n % 2 == 1)
            .filter(|&n| is_prime(n))
    }

    pub fn factors(number: u128) -> impl Iterator<Item = u128> {
        let mut primes = primes().take_while(move |p| p * p < number);
        let mut rem = number;
        let mut some_p = primes.next();

        std::iter::from_fn(move || loop {
            if rem == 1 {
                break None;
            } else if let Some(p) = some_p {
                if rem % p == 0 {
                    rem /= p;
                    break Some(p);
                } else {
                    some_p = primes.next()
                }
            } else {
                let ret = Some(rem);
                rem = 1;
                break ret;
            }
        })
    }

    /// Miller-Rabin test of primicity
    fn miller_rabin_test(number: u128) -> bool {
        // Early stop for 2 and evens
        if number == 2 {
            return true;
        } else if number < 2 || number % 2 == 0 {
            return false;
        };

        // > number = 2 ^ r * d + 1
        let (mut d, mut r): (u128, u128) = (number - 1, 0);
        while d % 2 == 0 {
            d /= 2;
            r += 1;
        }

        // Thoses are not optimals or exact boundry for the bases but eaily memorizable ones
        if number < 2_u128.pow(20) - 1 {
            vec![2, 3]
        } else if number < 2_u128.pow(32) - 1 {
            vec![2, 7, 61]
        } else if number < 2_u128.pow(40) - 1 {
            vec![2, 3, 5, 7, 11]
        } else if number < 2_u128.pow(48) - 1 {
            vec![2, 3, 5, 7, 11, 13, 17]
        } else if number < u64::MAX as u128 {
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
        } else {
            unimplemented!("miller_rabin_witnesses for number greater than u64::MAX")
        }
        .par_iter()
        .all(|&a| {
            let mut x = crate::math::modpow(a, d, number);
            if x == 1 || x == number - 1 {
                return true;
            }
            (1..r).any(|_| {
                x = (x * x) % number;
                x == number - 1
            })
        })
    }
}

pub mod math {
    /// Modular exponentiation
    /// Source: https://www.wikiwand.com/en/Modular_exponentiation
    pub fn modpow(n: u128, p: u128, m: u128) -> u128 {
        if (m - 1).checked_pow(2).is_none() {
            panic!("modpow: {} will overflow n", m)
        }
        let (mut n, mut p, mut r) = (n % m, p, 1);
        while p > 0 {
            if p % 2 == 1 {
                r = (r * n) % m
            }
            p >>= 1;
            n = n.pow(2) % m
        }
        r
    }
}
