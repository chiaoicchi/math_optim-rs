pub mod eratos;
pub mod gcd;
pub mod is_prime;
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
