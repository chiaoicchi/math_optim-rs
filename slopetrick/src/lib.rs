/// slope trick
///
/// # Example
///
/// ```
/// ```
///
use std::collections::BinaryHeap;
const INF: i64 = 1 << 60;
pub struct SlopeTrick {
    min: i64,
    l: BinaryHeap<i64>,
    r: BinaryHeap<i64>,
}
impl SlopeTrick {
    /// Initializer of slope trick by f(x) = 0.
    /// This funciton has a time complexity of O(1).
    pub fn new() -> Self {
        let mut l = BinaryHeap::new();
        let mut r = BinaryHeap::new();
        l.push(-INF);
        r.push(-INF);
        Self { min: 0, l, r }
    }
    /// Return minimum of f.
    /// This function has a time complexity of O(1).
    pub fn min(&self) -> i64 {
        self.min
    }
    /// Add const function g := a (a: const).
    /// f <- f + g.
    /// This function has a time complexity of O(1).
    pub fn add_const(&mut self, a: i64) {
        self.min += a;
    }
    /// Add absolute function g := max(x - a, 0).
    /// f <- f + g
    /// This function has a time complexity of O(log n).
    pub fn add_plus(&mut self, a: i64) {
        self.l.push(a);
        let x = self.l.pop().unwrap();
        self.min += x - a;
        self.r.push(-x);
    }
    /// Add absolute function g := min(x - a, 0).
    /// f <- f + g.
    /// This function has a time complexity og O(log n).
    pub fn add_minus(&mut self, a: i64) {
        self.r.push(-a);
        let x = self.r.pop().unwrap();
        self.min += a - x;
        self.l.push(x);
    }
    /// Add absolute function g := |x - a|.
    /// f <- f + g.
    /// This function has a time complexity of O(log n).
    pub fn add_abs(&mut self, a: i64) {
        self.add_plus(a);
        self.add_minus(a);
    }
}
