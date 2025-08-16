use algebra::Monoid;
/// monoid action.
///
/// # Example
///
/// ```
/// ```
///
pub trait MonoidAction: Monoid {
    type X: Clone;
    fn apply(val: &mut Self::X, map: &Self::S);
}
/// Dual Segment Tree.
///
/// # Example
///
/// ```
/// ```
///
pub struct DualSegTree<T: MonoidAction> {
    n: usize,
    m: usize,
    k: u32,
    vals: Vec<T::X>,
    maps: Vec<T::S>,
}
use std::ops::RangeBounds;
impl<T: MonoidAction> DualSegTree<T> {
    /// This is initilizer of `DualSegTree`.
    /// This function has a time complexity of O(n).
    pub fn new(n: usize, ini: &T::X) -> Self {
        let m = n.next_power_of_two();
        Self {
            n,
            m,
            k: m.trailing_zeros(),
            vals: vec![ini.clone(); n],
            maps: vec![T::E; 2 * m],
        }
    }
    /// Return `a_i`.
    /// This function has a time complexity of O(log n).
    pub fn get(&self, i: usize) -> T::X {
        use std::iter::successors;
        let mut val = self.vals[i].clone();
        successors(Some(i + self.m), |i| Some(i >> 1))
            .take_while(|i| *i > 0)
            .for_each(|i| T::apply(&mut val, &self.maps[i]));
        val
    }
    /// forall i in [l, r), update from `a_i` to `ai * f := f(a_i)`.
    /// This function has a time complexity of O(log n).
    pub fn range_apply(&mut self, range: impl RangeBounds<usize>, f: &T::S) {
        use std::mem::replace;
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
        // If monoid action is commutative, this propagation is not needed.
        (1..=self.k).rev().for_each(|i| {
            if (l >> i) << i != l {
                let g = replace(&mut self.maps[l >> i], T::E);
                self.maps[2 * (l >> i)] = T::op(&self.maps[2 * (l >> i)], &g);
                self.maps[2 * (l >> i) + 1] = T::op(&self.maps[2 * (l >> i) + 1], &g);
            }
            if (r >> i) << i != r {
                let g = replace(&mut self.maps[(r - 1) >> i], T::E);
                self.maps[2 * ((r - 1) >> i)] = T::op(&self.maps[2 * ((r - 1) >> i)], &g);
                self.maps[2 * ((r - 1) >> i) + 1] = T::op(&self.maps[2 * ((r - 1) >> i) + 1], &g);
            }
        });
        while l < r {
            if l & 1 == 1 {
                self.maps[l] = T::op(&self.maps[l], &f);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.maps[r] = T::op(&self.maps[r], &f);
            }
            l >>= 1;
            r >>= 1;
        }
    }
}
/// Constructor from iter to `DualSegTree`.
/// This function has a time complexity of O(n).
use std::iter::FromIterator;
impl<T: MonoidAction> FromIterator<T::X> for DualSegTree<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T::X>>(iter: I) -> DualSegTree<T> {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let m = n.next_power_of_two();
        Self {
            n,
            m,
            k: m.trailing_zeros(),
            vals: a,
            maps: vec![T::E; 2 * m],
        }
    }
}
