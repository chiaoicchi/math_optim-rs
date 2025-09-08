#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
#[repr(transparent)]
pub struct Vector(pub (i64, i64));
impl Vector {
    pub fn new(x: i64, y: i64) -> Self {
        Self((x, y))
    }
    pub fn x(&self) -> i64 {
        self.0 .0
    }
    pub fn y(&self) -> i64 {
        self.0 .1
    }
    pub fn add(&self, other: &Self) -> Self {
        Self((self.x() + other.x(), self.y() + other.y()))
    }
    pub fn sub(&self, other: &Self) -> Self {
        Self((self.x() - other.x(), self.y() - other.y()))
    }
    pub fn scalar(&self, a: &i64) -> Self {
        Self((a * self.x(), a * self.y()))
    }
    pub fn dot(&self, other: &Self) -> i64 {
        self.x() * other.x() + self.y() * other.y()
    }
    pub fn cross(&self, other: &Self) -> i64 {
        self.x() * other.y() - self.y() * other.x()
    }
    pub fn norm_pow2(&self) -> i64 {
        self.x() * self.x() + self.y() * self.y()
    }
}
impl From<(i64, i64)> for Vector {
    fn from(t: (i64, i64)) -> Self {
        Vector(t)
    }
}
impl From<Vector> for (i64, i64) {
    fn from(v: Vector) -> Self {
        v.0
    }
}
/// Return convex hull.
/// This function has a time complexity of O(n).
pub fn convex_hull(p: &[Vector]) -> Vec<Vector> {
    let mut p = p.to_vec();
    p.sort_unstable_by_key(|v| v.0);
    p.dedup();
    if p.is_empty() {
        return Vec::new();
    } else if p.len() == 1 {
        return vec![p[0]];
    } else if p.len() == 2 {
        return vec![p[0], p[1]];
    } else {
        let mut res = vec![p[0], p[1]];
        for p in &p[2..] {
            while res.len() > 1 && {
                let x = &res[res.len() - 2];
                let y = &res[res.len() - 1];
                (y.sub(x)).cross(&(p.sub(x))) >= 0
            } {
                res.pop();
            }
            res.push(*p);
        }
        let len = res.len();
        for p in p.iter().rev().skip(1) {
            while len < res.len() && {
                let x = &res[res.len() - 2];
                let y = &res[res.len() - 1];
                (y.sub(x)).cross(&(p.sub(x))) >= 0
            } {
                res.pop();
            }
            res.push(*p);
        }
        res.pop();
        res
    }
}
/// Calculate area of polygon.
/// This function has a time complexity of O(n).
pub fn polygon_area_mul2(p: &[Vector]) -> i64 {
    if p.len() < 3 {
        0
    } else {
        let a = &p[0];
        p.windows(2)
            .map(|x| x[0].sub(a).cross(&x[1].sub(a)).abs())
            .sum::<i64>()
    }
}
/// Compare 2 points with its argment.
/// This function has a time complexity of O(1)
pub fn arg_cmp(v: &Vector, w: &Vector) -> std::cmp::Ordering {
    ((v.y(), v.x()) < (0, 0))
        .cmp(&((w.y(), w.x()) < (0, 0)))
        .then_with(|| (v.y() * w.x()).cmp(&(v.x() * w.y())))
}
