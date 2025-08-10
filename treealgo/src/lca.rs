/// This is lowest common ancestor
///
/// # Exmample
///
/// ```
/// ```
///
pub struct LCA {
    n: usize,
    vals: Vec<usize>,
    d: Vec<usize>,
}
impl LCA {
    /// This is initializer of LCA.
    /// This function has a time complexity of O(n log n).
    pub fn build(r: usize, n: usize, e: &[(usize, usize)]) -> Self {
        assert!(r < n);
        assert_eq!(n, e.len() + 1);
        let mut g = vec![vec![]; n];
        for &(u, v) in e {
            g[u].push(v);
            g[v].push(u);
        }
        let mut vals = vec![!0; n];
        let mut d = vec![!0; n];
        let mut stack = vec![r];
        vals[r] = r;
        d[r] = 0;
        while let Some(u) = stack.pop() {
            for &v in &g[u] {
                if d[v] == !0 {
                    vals[v] = u;
                    d[v] = d[u] + 1;
                    stack.push(v);
                }
            }
        }
        (1..).take_while(|i| 1 << i < n).for_each(|i| {
            (0..n).for_each(|j| vals.push(vals[(i - 1) * n + vals[(i - 1) * n + j]]))
        });
        Self { n, vals, d }
    }
    /// Return lowest common ancestor.
    /// This function has a time complexity of O(log n).
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        assert!(u < self.n);
        assert!(v < self.n);
        use std::mem::swap;
        if self.d[u] > self.d[v] {
            swap(&mut u, &mut v);
        }
        let d = self.d[v] - self.d[u];
        for (i, vals) in self.vals.chunks_exact(self.n).enumerate() {
            if (d >> i) & 1 == 1 {
                v = vals[v];
            }
        }
        if u == v {
            u
        } else {
            for vals in self.vals.chunks_exact(self.n).rev() {
                if vals[u] == vals[v] {
                    continue;
                }
                u = vals[u];
                v = vals[v];
            }
            self.vals[u]
        }
    }
    /// Return distance between `u` and `v`.
    /// This function has a time complexity of O(log n).
    pub fn d(&self, u: usize, v: usize) -> usize {
        let lca = self.lca(u, v);
        self.d[u] + self.d[v] - 2 * self.d[lca]
    }
}
