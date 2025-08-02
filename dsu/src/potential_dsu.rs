use algebra::Group;
/// potential disjoint set union.
///
/// # Example
///
/// ```
/// ```
///
pub struct PotentialDSU<T: Group> {
    parents: Vec<i32>,
    potentials: Vec<T::S>,
    count: usize,
}
impl<T: Group> PotentialDSU<T> {
    /// This is initializer of `Potential DSU`.
    /// There are `n` singletons.
    /// This function has a time complexity of O(n).
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            potentials: vec![T::e(); n],
            count: n,
        }
    }
    /// Return the representative of the et which contains `v`,
    /// and potentials from `rep(v)` to `v`.
    /// If `u` and `v` are in a same set, `rep(u)` == `rep(v)`.
    /// This function has a time complexity of O(n log n).
    pub fn rep(&self, mut v: usize) -> (usize, T::S) {
        let mut pot = self.potentials[v].clone();
        while self.parents[v] >= 0 {
            v = self.parents[v] as usize;
            pot = T::op(&self.potentials[v], &pot);
        }
        (v, pot)
    }
    /// Return potential from `u` to `v`.
    /// This function has a time complexity of O(log n).
    pub fn potential(&self, u: usize, v: usize) -> Option<T::S> {
        let (u, pot_u) = self.rep(u);
        let (v, pot_v) = self.rep(v);
        if u == v {
            Some(T::op(&T::inv(&pot_u), &pot_v))
        } else {
            None
        }
    }
    /// Unite two sets which contain `u` or `v` with potential `p` from `u` to `v`.
    /// If argments are not invalid return false, else return true.
    /// This function has a time complexity of O(n log n).
    pub fn unite(&mut self, u: usize, v: usize, p: &T::S) -> bool {
        use std::mem::swap;
        let (mut u, pot_u) = self.rep(u);
        let (mut v, pot_v) = self.rep(v);
        if u == v {
            T::op(&pot_u, p) == pot_v
        } else {
            let mut p = T::op(&T::op(&pot_u, p), &T::inv(&pot_v));
            if self.parents[u] > self.parents[v] {
                swap(&mut u, &mut v);
                p = T::inv(&p);
            }
            self.parents[u] += self.parents[v];
            self.parents[v] = u as i32;
            self.potentials[v] = p;
            self.count -= 1;
            true
        }
    }
    /// Return the size of set which contains `v`.
    /// This function has a time complexity of O(log n).
    pub fn size(&self, v: usize) -> usize {
        -self.parents[self.rep(v).0] as usize
    }
    /// Return how many sets are there.
    /// This function has a time complexity of O(1).
    pub fn count(&self) -> usize {
        self.count
    }
}
