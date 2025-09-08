use algebra::Monoid;
pub trait Action<V: Monoid, F: Monoid> {
    fn apply(val: &mut <V as Monoid>::S, func: &<F as Monoid>::S);
}
/// Lazy Segment Tree.
pub struct LazySegTree<V: Monoid, F: Monoid, A: Action<V, F>> {
    n: usize,
    m: usize,
    k: u32,
    vals: Vec<<V as Monoid>::S>,
    maps: Vec<<F as Monoid>::S>,
    _marker: std::marker::PhantomData<A>,
}
impl<V: Monoid, F: Monoid, A: Action<V, F>> LazySegTree<V, F, A> {
    /// This is initilizer of `LazySegTree`.
    /// This function has a time complexity of O(n).
    pub fn new(n: usize) -> Self {
        let m = n.next_power_of_two();
        Self {
            n,
            m,
            k: m.trailing_zeros(),
            vals: vec![<V as Monoid>::E; 2 * m],
            maps: vec![<F as Monoid>::E; 2 * m],
            _marker: std::marker::PhantomData,
        }
    }
    /// Return the value of index `i`.
    /// This function has a time complexity of O(log n).
    pub fn get(&mut self, i: usize) -> &<V as Monoid>::S {
        assert!(i < self.n);
        let i = i + self.m;
        for v in (1..=self.k).rev() {
            self.push(i >> v);
        }
        &self.vals[i]
    }
    /// Update the value of index `i` to `x`.
    /// This function has a time complexity of O(log n).
    pub fn update(&mut self, i: usize, x: &<V as Monoid>::S) {
        assert!(i < self.n);
        let i = i + self.m;
        for v in (1..=self.k).rev() {
            self.push(i >> v);
        }
        self.vals[i] = x.clone();
        for v in 1..=self.k {
            self.vals[i >> v] =
                <V as Monoid>::op(&self.vals[2 * (i >> v)], &self.vals[2 * (i >> v) + 1])
        }
    }
    /// Return the fold by op in `range`.
    /// This function has a time complexity of O(log n).
    pub fn range_fold(&mut self, range: impl std::ops::RangeBounds<usize>) -> <V as Monoid>::S {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let mut l = match range.start_bound() {
            Unbounded => 0,
            Included(l) => *l,
            Excluded(l) => l + 1,
        } + self.m;
        let mut r = match range.end_bound() {
            Unbounded => self.n,
            Included(r) => r + 1,
            Excluded(r) => *r,
        } + self.m;
        assert!(l <= r);
        assert!(l < self.n + self.m);
        assert!(r <= self.n + self.m);
        for v in (1..=self.k).rev() {
            if (l >> v) << v != l {
                self.push(l >> v);
            }
            if (r >> v) << v != r {
                self.push((r - 1) >> v);
            }
        }
        let mut left = <V as Monoid>::E;
        let mut right = <V as Monoid>::E;
        while l < r {
            if l & 1 == 1 {
                left = <V as Monoid>::op(&left, &self.vals[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = <V as Monoid>::op(&self.vals[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        <V as Monoid>::op(&left, &right)
    }
    /// Apply `f` to the value which index is in `range`.
    /// This function has a time complexity of O(log n).
    pub fn range_apply(&mut self, range: impl std::ops::RangeBounds<usize>, f: &<F as Monoid>::S) {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(l) => *l,
            Excluded(l) => l + 1,
        } + self.m;
        let r = match range.end_bound() {
            Unbounded => self.n,
            Included(r) => r + 1,
            Excluded(r) => *r,
        } + self.m;
        assert!(l <= r);
        assert!(l < self.n + self.m);
        assert!(r <= self.n + self.m);
        for v in (1..=self.k).rev() {
            if (l >> v) << v != l {
                self.push(l >> v);
            }
            if (r >> v) << v != r {
                self.push((r - 1) >> v);
            }
        }
        {
            let (mut l, mut r) = (l, r);
            while l < r {
                if l & 1 == 1 {
                    A::apply(&mut self.vals[l], f);
                    self.maps[l] = <F as Monoid>::op(&self.maps[l], f);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    A::apply(&mut self.vals[r], f);
                    self.maps[r] = <F as Monoid>::op(&self.maps[r], f);
                }
                l >>= 1;
                r >>= 1;
            }
        }
        for v in 1..=self.k {
            if (l >> v) << v != l {
                self.vals[l >> v] =
                    <V as Monoid>::op(&self.vals[2 * (l >> v)], &self.vals[2 * (l >> v) + 1]);
            }
            if (r >> v) << v != r {
                self.vals[(r - 1) >> v] = <V as Monoid>::op(
                    &self.vals[2 * ((r - 1) >> v)],
                    &self.vals[2 * ((r - 1) >> v) + 1],
                );
            }
        }
    }
    /// Push action of index `k`.
    /// Apply its action and push to its children.
    /// This function has a time complexity of O(1).
    fn push(&mut self, k: usize) {
        use std::mem::replace;
        let f = replace(&mut self.maps[k], <F as Monoid>::E);
        A::apply(&mut self.vals[2 * k], &f);
        A::apply(&mut self.vals[2 * k + 1], &f);
        self.maps[2 * k] = <F as Monoid>::op(&self.maps[2 * k], &f);
        self.maps[2 * k + 1] = <F as Monoid>::op(&self.maps[2 * k + 1], &f);
    }
}
/// Construct from iter to `lazysegtree`
/// This function has a time complexity of O(n).
impl<V: Monoid, F: Monoid, A: Action<V, F>> std::iter::FromIterator<<V as Monoid>::S>
    for LazySegTree<V, F, A>
{
    #[inline(always)]
    fn from_iter<I: IntoIterator<Item = <V as Monoid>::S>>(iter: I) -> LazySegTree<V, F, A> {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let m = n.next_power_of_two();
        let mut vals = vec![<V as Monoid>::E; 2 * m];
        vals[m..m + n].clone_from_slice(&a);
        for i in (1..m).rev() {
            vals[i] = <V as Monoid>::op(&vals[2 * i], &vals[2 * i + 1]);
        }
        Self {
            n,
            m,
            k: m.trailing_zeros(),
            vals,
            maps: vec![<F as Monoid>::E; 2 * m],
            _marker: std::marker::PhantomData,
        }
    }
}
