pub mod dual;
pub mod lazy;
use algebra::Monoid;
/// Segment Tree
///
/// # Example
///
/// ```
/// ```
///
pub struct SegTree<T: Monoid> {
    n: usize,
    m: usize,
    vals: Vec<T::S>,
}
use std::ops::RangeBounds;
impl<T: Monoid> SegTree<T> {
    /// This is initializer of `SegTree`.
    /// This function has a time complexity of O(n).
    pub fn new(n: usize) -> Self {
        let m = n.next_power_of_two();
        Self {
            n,
            m,
            vals: vec![T::E; 2 * m],
        }
    }
    /// Update `i` th, from `a_i` to `x`.
    /// This function has a time complexity of O(log n).
    pub fn set_at(&mut self, i: usize, x: &T::S) {
        use std::iter::successors;
        assert!(i < self.n);
        let mut i = i + self.m;
        self.vals[i] = x.clone();
        i >>= 1;
        successors(Some(i), |i| Some(i >> 1))
            .take_while(|i| *i > 0)
            .for_each(|i| self.vals[i] = T::op(&self.vals[2 * i], &self.vals[2 * i + 1]));
    }
    /// Return `a_i`.
    /// This function has a time complexity of O(1).
    pub fn get_at(&self, i: usize) -> &T::S {
        assert!(i < self.n);
        &self.vals[i + self.m]
    }
    /// Return `op(a_l, ..., a_r)`.
    /// This function has a time complexity of O(log n).
    pub fn range_fold(&self, range: impl RangeBounds<usize>) -> T::S {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let mut l = match range.start_bound() {
            Unbounded => 0,
            Included(x) => *x,
            Excluded(x) => x + 1,
        } + self.m;
        let mut r = match range.end_bound() {
            Unbounded => self.n,
            Included(x) => x + 1,
            Excluded(x) => *x,
        } + self.m;
        let mut left = T::E;
        let mut right = T::E;
        while l < r {
            if l & 1 == 1 {
                left = T::op(&left, &self.vals[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = T::op(&self.vals[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(&left, &right)
    }
}
/// Constructor from iter to `SegTree`.
/// This function has a time complexity of O(n).
impl<T: Monoid> FromIterator<T::S> for SegTree<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T::S>>(iter: I) -> SegTree<T> {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let m = n.next_power_of_two();
        let mut vals = vec![T::E; 2 * m];
        vals[m..m + n].clone_from_slice(&a);
        (1..m)
            .rev()
            .for_each(|i| vals[i] = T::op(&vals[2 * i], &vals[2 * i + 1]));
        Self { n, m, vals }
    }
}
