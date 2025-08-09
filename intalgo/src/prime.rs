/// Return whether `n` is prime.
/// This function has a time complexity of O(log n).
pub fn is_prime(n: u64) -> bool {
    use std::iter::successors;
    if n == 0 || n == 1 {
        false
    } else if n == 2 {
        true
    } else if n & 1 == 0 {
        false
    } else {
        let n = n as u128;
        let r = (n - 1).trailing_zeros();
        let d = (n - 1) >> r;
        (if n < 4_759_123_141 {
            vec![2, 7, 61]
        } else {
            vec![2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022]
        })
        .iter()
        .filter(|x| **x < n)
        .all(|x| {
            let mut x = *x;
            let mut pow = 1;
            successors(Some(d), |d| Some(*d >> 1))
                .take_while(|d| *d > 0)
                .for_each(|d| {
                    if d & 1 == 1 {
                        pow *= x;
                        pow %= n;
                    }
                    x *= x;
                    x %= n;
                });
            if pow == 1 || pow == n - 1 {
                true
            } else {
                (1..r).any(|_| {
                    pow *= pow;
                    pow %= n;
                    pow == n - 1
                })
            }
        })
    }
}
// Return factorize of a.
/// This function has a time complexity of O(n^1/4 log(n))
use super::gcd::gcd;
pub fn factorize(mut a: u64) -> Vec<u64> {
    assert!(a > 0);
    let two = a.trailing_zeros();
    let mut res = vec![2; two as usize];
    a >>= two;
    while a % 3 == 0 {
        res.push(3);
        a /= 3;
    }
    if a == 1 {
        res
    } else {
        let mut i = res.len();
        res.push(a);
        while i < res.len() {
            let a = res[i];
            if is_prime(a) {
                i += 1;
                continue;
            }
            'LOOP: for t in 1.. {
                let mut x = t as u64;
                let mut y = ((x as u128 * x as u128 + t) % a as u128) as u64;
                loop {
                    let g = gcd(y + a - x, a);
                    if g == 0 || g == a {
                        break;
                    }
                    if g != 1 {
                        res[i] /= g;
                        res.push(g);
                        break 'LOOP;
                    }
                    x = ((x as u128 * x as u128 + t) % a as u128) as u64;
                    y = ((y as u128 * y as u128 + t) % a as u128) as u64;
                    y = ((y as u128 * y as u128 + t) % a as u128) as u64;
                }
            }
        }
        res.sort_unstable();
        res
    }
}
/// Return primitive root of prime number p.
/// This function has a time complexity of O(p^1/4 log p).
pub fn primitive_root(p: u64) -> u64 {
    use std::iter::successors;
    if p == 2 {
        return 1;
    } else {
        let mut factor = factorize(p - 1);
        factor.dedup();
        'LOOP: for g in 2..p {
            for f in &factor {
                let mut g = g;
                if successors(Some((p - 1) / f), |i| Some(i >> 1))
                    .take_while(|i| *i > 0)
                    .fold(1, |acc, i| {
                        let res = if i & 1 == 1 {
                            (acc as u128 * g as u128 % p as u128) as u64
                        } else {
                            acc
                        };
                        g = (g as u128 * g as u128 % p as u128) as u64;
                        res
                    })
                    == 1
                {
                    continue 'LOOP;
                }
            }
            return g;
        }
    }
    unreachable!();
}
