use algebra::Monoid;
/// monoid action.
///
/// # Example
///
/// ```
/// ```
///
pub trait MonoidAction2Monoid: Monoid {
    type X: Clone;
    const E: Self::X;
    fn op(lhs: &Self::X, rhs: &Self::X) -> Self::X;
    fn apply(val: &mut Self::X, map: &Self::S);
}
/// Dual Segment Tree.
///
/// # Example
///
/// ```
/// ```
///
pub struct LazySegTree<T: MonoidAction2Monoid> {
    n: usize,
    m: usize,
    k: u32,
    vals: Vec<T::X>,
    maps: Vec<T::S>,
}
use std::ops::RangeBounds;
impl<T: MonoidAction2Monoid> LazySegTree<T> {
    /// This is initilizer of `DualSegTree`.
    /// This function has a time complexity of O(n).
    pub fn new(n: usize) -> Self {
        let m = n.next_power_of_two();
        Self {
            n,
            m,
            k: m.trailing_zeros(),
            vals: vec![<T as MonoidAction2Monoid>::E; n],
            maps: vec![<T as Monoid>::E; 2 * m],
        }
    }
    /// Return `a_i`.
    /// This function has a time complexity of O(log n).
    pub fn get(&mut self, i: usize) -> T::X {
        assert!(i < self.n);
        use std::mem::replace;
        let i = i + self.m;
        (1..=self.k).rev().for_each(|j| {
            let f = replace(&mut self.maps[i >> j], <T as Monoid>::E);
            T::apply(&mut self.vals[2 * (i >> j)], &f);
            T::apply(&mut self.vals[2 * (i >> j) + 1], &f);
            self.maps[2 * (i >> j)] = <T as Monoid>::op(&self.maps[2 * (i >> j)], &f);
            self.maps[2 * (i >> j) + 1] = <T as Monoid>::op(&self.maps[2 * (i >> j) + 1], &f);
        });
        self.vals[i].clone()
    }
    pub fn update(&mut self, i: usize, x: &T::X) {
        assert!(i < self.n);
        use std::mem::replace;
        let i = i + self.m;
        (1..=self.k).rev().for_each(|j| {
            let f = replace(&mut self.maps[i >> j], <T as Monoid>::E);
            T::apply(&mut self.vals[2 * (i >> j)], &f);
            self.maps[2 * (i >> j)] = <T as Monoid>::op(&self.maps[2 * (i >> j)], &f);
            T::apply(&mut self.vals[2 * (i >> j) + 1], &f);
            self.maps[2 * (i >> j) + 1] = <T as Monoid>::op(&self.maps[2 * (i >> j) + 1], &f);
        });
        self.vals[i] = x.clone();
        (1..=self.k).for_each(|j| {
            self.vals[i >> j] = <T as MonoidAction2Monoid>::op(
                &self.vals[2 * (i >> j)],
                &self.vals[2 * (i >> j) + 1],
            )
        });
    }
    /// Return `op(a_l, ..., a_{r - 1})`.
    /// This function has time complexity of O(log n).
    pub fn range_fold(&mut self, range: impl RangeBounds<usize>) -> T::X {
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
        assert!(l < self.n + self.m);
        assert!(l <= self.n + self.m);
        (1..=self.k).rev().for_each(|i| {
            if (l >> i) << i != l {
                let f = replace(&mut self.maps[l >> i], <T as Monoid>::E);
                T::apply(&mut self.vals[2 * (l >> i)], &f);
                self.maps[2 * (l >> i)] = <T as Monoid>::op(&self.maps[2 * (l >> i)], &f);
                T::apply(&mut self.vals[2 * (l >> i) + 1], &f);
                self.maps[2 * (l >> i) + 1] = <T as Monoid>::op(&self.maps[2 * (l >> i) + 1], &f);
            }
            if (r >> i) << i != r {
                let f = replace(&mut self.maps[(r - 1) >> i], <T as Monoid>::E);
                T::apply(&mut self.vals[2 * ((r - 1) >> i)], &f);
                self.maps[2 * ((r - 1) >> i)] =
                    <T as Monoid>::op(&self.maps[2 * ((r - 1) >> i)], &f);
                T::apply(&mut self.vals[2 * ((r - 1) >> i) + 1], &f);
                self.maps[2 * ((r - 1) >> i) + 1] =
                    <T as Monoid>::op(&self.maps[2 * ((r - 1) >> i) + 1], &f);
            }
        });
        let mut left = <T as MonoidAction2Monoid>::E;
        let mut right = <T as MonoidAction2Monoid>::E;
        while l < r {
            if l & 1 == 1 {
                left = <T as MonoidAction2Monoid>::op(&left, &self.vals[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = <T as MonoidAction2Monoid>::op(&self.vals[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        <T as MonoidAction2Monoid>::op(&left, &right)
    }
    /// forall i in [l, r), update from `a_i` to `ai * f := f(a_i)`.
    /// This function has a time complexity of O(log n).
    pub fn range_apply(&mut self, range: impl RangeBounds<usize>, f: &T::S) {
        use std::mem::replace;
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(x) => *x,
            Excluded(x) => x + 1,
        } + self.m;
        let r = match range.end_bound() {
            Unbounded => self.n,
            Included(x) => x + 1,
            Excluded(x) => *x,
        } + self.m;
        assert!(l < self.n + self.m);
        assert!(l <= self.n + self.m);
        (1..=self.k).rev().for_each(|i| {
            if (l >> i) << i != l {
                let g = replace(&mut self.maps[l >> i], <T as Monoid>::E);
                T::apply(&mut self.vals[2 * (l >> i)], &g);
                self.maps[2 * (l >> i)] = <T as Monoid>::op(&self.maps[2 * (l >> i)], &g);
                T::apply(&mut self.vals[2 * (l >> i) + 1], &g);
                self.maps[2 * (l >> i) + 1] = <T as Monoid>::op(&self.maps[2 * (l >> i) + 1], &g);
            }
            if (r >> i) << i != r {
                let g = replace(&mut self.maps[(r - 1) >> i], <T as Monoid>::E);
                T::apply(&mut self.vals[2 * ((r - 1) >> i)], &g);
                self.maps[2 * ((r - 1) >> i)] =
                    <T as Monoid>::op(&self.maps[2 * ((r - 1) >> i)], &g);
                T::apply(&mut self.vals[2 * ((r - 1) >> i) + 1], &g);
                self.maps[2 * ((r - 1) >> i) + 1] =
                    <T as Monoid>::op(&self.maps[2 * ((r - 1) >> i) + 1], &g);
            }
        });
        {
            let (mut l, mut r) = (l, r);
            while l < r {
                if l & 1 == 1 {
                    T::apply(&mut self.vals[l], f);
                    self.maps[l] = <T as Monoid>::op(&self.maps[l], f);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    T::apply(&mut self.vals[r], f);
                    self.maps[r] = <T as Monoid>::op(&self.maps[r], f);
                }
                l >>= 1;
                r >>= 1;
            }
        }
        (1..=self.k).for_each(|i| {
            if (l >> i) << i != l {
                self.vals[l >> i] = <T as MonoidAction2Monoid>::op(
                    &self.vals[2 * (l >> i)],
                    &self.vals[2 * (l >> i) + 1],
                );
            }
            if (r >> i) << i != r {
                self.vals[(r - 1) >> i] = <T as MonoidAction2Monoid>::op(
                    &self.vals[2 * ((r - 1) >> i)],
                    &self.vals[2 * ((r - 1) >> i) + 1],
                );
            }
        })
    }
}
/// Constructor from iter to `DualSegTree`.
/// This function has a time complexity of O(n).
impl<T: MonoidAction2Monoid> FromIterator<T::X> for LazySegTree<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T::X>>(iter: I) -> LazySegTree<T> {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let m = n.next_power_of_two();
        let mut vals = vec![<T as MonoidAction2Monoid>::E; 2 * m];
        vals[m..m + n].clone_from_slice(&a);
        (1..m)
            .rev()
            .for_each(|i| vals[i] = <T as MonoidAction2Monoid>::op(&vals[2 * i], &vals[2 * i + 1]));
        Self {
            n,
            m,
            k: m.trailing_zeros(),
            vals,
            maps: vec![<T as Monoid>::E; 2 * m],
        }
    }
}
