/// Return `gcd(a, b)`.
/// This function has time complexity of O(log max(a, b)).
pub fn gcd(a: u64, b: u64) -> u64 {
    use std::iter::successors;
    successors(
        Some((a, b)),
        |(a, b)| if *b == 0 { None } else { Some((*b, a % b)) },
    )
    .last()
    .unwrap()
    .0
}
/// Return `lcm(a, b)`.
/// This function has time compleity of O(log max(a, b)).
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
