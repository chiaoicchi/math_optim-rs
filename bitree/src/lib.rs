use algebra::Monoid;
/// Binary Index Tree
///
/// # Example
///
/// ```
/// ```
///
pub struct BITree<T: Monoid> {
    n: usize,
    vals: Vec<T::S>,
}
impl<T: Monoid> BITree<T> {
    /// This is initializer of `BITree`.
    /// This fucntion has a time complexity of O(1).
    pub fn new() -> Self {
        Self {
            n: 1,
            vals: vec![T::E],
        }
    }
    /// Push `x` to `BIT`.
    /// This function has a time complexity of O(log n).
    pub fn push(&mut self, mut x: T::S) {
        let lsb = self.n & self.n.wrapping_neg();
        let mut d = 1;
        while d < lsb {
            x = T::op(&x, &self.vals[self.n - d]);
            d <<= 1;
        }
        self.vals.push(x);
        self.n += 1;
    }
    /// Update `i` th, from `a_i` to `op(a_i, x)`.
    /// This function has a time complexity of O(log n).
    pub fn op_at(&mut self, i: usize, x: &T::S) {
        use std::iter::successors;
        let n = self.n;
        successors(Some(i + 1), |i| Some(i + (i & i.wrapping_neg())))
            .take_while(|&i| i < n)
            .for_each(|i| self.vals[i] = T::op(&self.vals[i], x));
    }
    /// Return op(a_0, ..., a_{i - 1}).
    /// This function has a time complexity of O(log n).
    pub fn prefix_fold(&self, i: usize) -> T::S {
        use std::iter::successors;
        successors(Some(i), |i| Some(i - (i & i.wrapping_neg())))
            .take_while(|i| *i != 0)
            .map(|i| &self.vals[i])
            .fold(T::E, |acc, a| T::op(&acc, &a))
    }
}
/// Constructor from iter to `BIT`.
/// This function has a time complexity of O(n).
impl<T: Monoid> FromIterator<T::S> for BITree<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T::S>>(iter: I) -> BITree<T> {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let mut vals = vec![T::E; n + 1];
        for (i, a) in a.iter().enumerate() {
            let i = i + 1;
            vals[i] = T::op(&vals[i], a);
            let lsb = i & i.wrapping_neg();
            if i + lsb <= n {
                vals[i + lsb] = T::op(&vals[i + lsb], &vals[i]);
            }
        }
        Self { n: n + 1, vals }
    }
}
