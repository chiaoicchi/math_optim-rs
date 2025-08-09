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
