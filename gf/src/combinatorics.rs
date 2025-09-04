use super::{gf, GF};
/// combinatorics.
///
/// # Example
///
/// ```
/// ```
///
pub struct Combinatorics<const MOD: u32> {
    n: u32,
    facts: Vec<GF<MOD>>,
    inv_facts: Vec<GF<MOD>>,
}
impl<const MOD: u32> Combinatorics<MOD> {
    /// This function build Combinatorics struct.
    /// This function has a time complexity of O(n).
    pub fn build(n: u32) -> Self {
        let mut facts = Vec::with_capacity(n as usize);
        facts.push(gf!(1, MOD));
        for i in 1..n {
            facts.push(facts.last().unwrap() * gf!(i));
        }
        let mut inv_facts = Vec::with_capacity(n as usize);
        inv_facts.push(facts.last().unwrap().inv());
        for i in (1..n).rev() {
            inv_facts.push(inv_facts.last().unwrap() * gf!(i));
        }
        inv_facts.reverse();
        Self {
            n,
            facts,
            inv_facts,
        }
    }
    /// Return `n!`.
    /// This function has a time complexity of O(1).
    pub fn factorial(&self, n: u32) -> GF<MOD> {
        assert!(n < self.n);
        self.facts[n as usize]
    }
    /// Return binominal of `(n r)`.
    /// This function has a time complexity of O{1}.
    pub fn binom(&self, n: u32, r: u32) -> GF<MOD> {
        assert!(n < self.n);
        assert!(r <= n);
        self.facts[n as usize] * self.inv_facts[r as usize] * self.inv_facts[(n - r) as usize]
    }
}
