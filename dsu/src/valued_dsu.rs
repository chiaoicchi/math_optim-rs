use algebra::ComMonoid;
pub struct ValuedDSU<T: ComMonoid> {
    parents: Vec<i32>,
    count: usize,
    vals: Vec<T::S>,
}
impl<T: ComMonoid> ValuedDSU<T> {
    /// This is initializer of `DSU`.
    /// There are `n` singletons.
    /// This function has a time complexity of O(n).
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            count: n,
            vals: vec![T::E; n],
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
    /// Return the value of `v`.
    /// This function has a time complexity of O(log n).
    pub fn val(&self, v: usize) -> &T::S {
        &self.vals[self.rep(v)]
    }
    /// Update the value of `v`.
    /// This function has a time complexity of O(log n).
    pub fn update(&mut self, v: usize, x: &T::S) {
        let v = self.rep(v);
        self.vals[v] = x.clone();
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
            self.vals[u] = T::op(&self.vals[u], &self.vals[v]);
            self.vals[v] = T::E;
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
