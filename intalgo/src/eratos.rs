/// Sieve of Eratosthenes
///
/// # Exmample
///
/// ```
/// ```
///
pub struct SieveEratos {
    n: usize,
    min_fact: Vec<usize>,
}
impl SieveEratos {
    /// This is initializer of SeiveEratosthenes of less than and equal n.
    /// This fucntion has a time complexity of O(n log log n).
    pub fn build(n: usize) -> Self {
        let mut min_fact = (0..=n).collect::<Vec<_>>();
        (2..=n / 2).for_each(|i| {
            if min_fact[i] == i {
                min_fact
                    .iter_mut()
                    .skip(2 * i)
                    .step_by(i)
                    .for_each(|j| *j = i);
            }
        });
        Self { n, min_fact }
    }
    /// Return whether `n` is prime or not.
    /// This function has a time complexity of O(1).
    pub fn is_prime(&self, n: usize) -> bool {
        assert!(n <= self.n);
        n > 1 && self.min_fact[n] == n
    }
    /// Return primes less than and equal n.
    /// This function has a time complexity of O(n).
    pub fn primes(&self, n: usize) -> Vec<usize> {
        assert!(n <= self.n);
        if n < 2 {
            vec![]
        } else {
            (2..=n).filter(|i| self.min_fact[*i] == *i).collect()
        }
    }
}
