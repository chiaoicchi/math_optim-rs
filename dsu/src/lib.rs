/// disjoint set union.
///
/// # Example
///
/// ```
/// use dsu::DSU;
/// let mut dsu = DSU::new(4);
/// assert!(!dsu.is_same(0, 2));
/// dsu.unite(0, 2);
/// assert!(dsu.is_same(0, 2));
/// dsu.unite(0, 1);
/// assert!(dsu.is_same(1, 2));
/// ```
///
pub struct DSU {
    parents: Vec<i32>,
    count: usize,
}
impl DSU {
    /// This is initializer of `DSU`.
    /// There are `n` singletons.
    /// This function has a time complexity of O(n).
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            count: n,
        }
    }
    /// Return the representative of the set which contains `v`.
    /// If `u` and `v` are in a same set, `rep(u)` == `rep(v)`.
    /// This function has a time complexity of O(log n).
    pub fn rep(&self, mut v: usize) -> usize {
        while self.parents[v] >= 0 {
            v = self.parents[v] as usize;
        }
        v
    }
    /// Return whether `u` and `v` are in the same set.
    /// This function has a time complexity of O(log n).
    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.rep(u) == self.rep(v)
    }
    /// Unite two sets which contain `u` or `v`.
    /// If `u` and `v` are already united return false, otherwise return true.
    /// This function has a time complexity of O(log n).
    pub fn unite(&mut self, u: usize, v: usize) -> bool {
        use std::mem::swap;
        let mut u = self.rep(u);
        let mut v = self.rep(v);
        if u == v {
            false
        } else {
            if self.parents[u] > self.parents[v] {
                swap(&mut u, &mut v);
            }
            self.parents[u] += self.parents[v];
            self.parents[v] = u as i32;
            self.count -= 1;
            true
        }
    }
    /// Return the size of set which contains `v`.
    /// This function has a time complexity of O(log n).
    pub fn size(&self, v: usize) -> usize {
        -self.parents[self.rep(v)] as usize
    }
    /// Return how many sets are there.
    /// This function has a time complexity of O(1).
    pub fn count(&self) -> usize {
        self.count
    }
}
