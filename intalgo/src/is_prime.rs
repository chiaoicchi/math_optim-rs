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
