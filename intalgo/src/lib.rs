pub mod eratos;
pub mod prime;
/// Return all divisors.
/// This function has a time complexity of O(sqrt n).
pub fn divisors(n: u64) -> Vec<u64> {
    let mut prefix = vec![];
    let mut suffix = vec![];
    (1..).take_while(|i| i * i <= n).for_each(|i| {
        if n % i == 0 {
            prefix.push(i);
            if i * i != n {
                suffix.push(n / i);
            }
        }
    });
    prefix.extend(suffix.iter().rev());
    prefix
}
/// Return a ^ n mod m.
/// This function has a time complexity of O(log n).
pub fn pow_mod<T: std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + From<u8> + Copy>(
    mut a: T,
    mut n: u64,
    m: T,
) -> T {
    let mut res = T::from(1u8);
    a = a % m;
    while n > 0 {
        if n & 1 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        n >>= 1;
    }
    res
}
/// Return `gcd(a, b)`.
/// This function has time complexity of O(log max(a, b)).
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    use std::mem::swap;
    if a == 0 || b == 0 {
        return a + b;
    }
    let x = a.trailing_zeros();
    let y = b.trailing_zeros();
    a >>= x;
    b >>= y;
    while a != b {
        let x = (a ^ b).trailing_zeros();
        if a < b {
            swap(&mut a, &mut b);
        }
        a = (a - b) >> x;
    }
    a << x.min(y)
}
/// Return `lcm(a, b)`.
/// This function has time compleity of O(log max(a, b)).
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
