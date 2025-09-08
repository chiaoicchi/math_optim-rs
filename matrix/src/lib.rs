#[derive(Clone)]
pub struct Matrix<T: Copy> {
    h: usize,
    w: usize,
    val: Box<[T]>,
}
impl<T: Copy> Matrix<T> {
    pub fn new(h: usize, w: usize, a: &[Vec<T>]) -> Self {
        let mut val = Vec::with_capacity(h * w);
        for a in a {
            val.extend(a.iter());
        }
        Self {
            h,
            w,
            val: val.to_vec().into_boxed_slice(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.val.chunks_exact(self.w)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.val.chunks_exact_mut(self.w)
    }
}
impl<T: Copy + std::ops::Add<Output = T>> Matrix<T> {
    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.h, other.h);
        assert_eq!(self.w, other.w);
        let mut res = self.val.clone();
        for (res, other) in res.iter_mut().zip(other.val.iter()) {
            *res = *res + *other;
        }
        Self {
            h: self.h,
            w: self.w,
            val: res,
        }
    }
    pub fn add_assign(&mut self, other: &Self) {
        assert_eq!(self.h, other.h);
        assert_eq!(self.w, other.w);
        for (res, other) in self.val.iter_mut().zip(other.val.iter()) {
            *res = *res + *other;
        }
    }
}
impl<T: Copy + std::ops::Sub<Output = T>> Matrix<T> {
    pub fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.h, other.h);
        assert_eq!(self.w, other.w);
        let mut res = self.val.clone();
        for (res, other) in res.iter_mut().zip(other.val.iter()) {
            *res = *res - *other;
        }
        Self {
            h: self.h,
            w: self.w,
            val: res,
        }
    }
    pub fn sub_assign(&mut self, other: &Self) {
        assert_eq!(self.h, other.h);
        assert_eq!(self.w, other.w);
        for (res, other) in self.val.iter_mut().zip(other.val.iter()) {
            *res = *res - *other;
        }
    }
}
impl<T: Copy + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + From<u8>> Matrix<T> {
    pub fn e(n: usize) -> Self {
        let mut val = vec![0u8.into(); n * n];
        for i in 0..n {
            val[i * n] = 1u8.into();
        }
        Self {
            h: n,
            w: n,
            val: val.into_boxed_slice(),
        }
    }
    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.w, other.h);
        let mut res = Self {
            h: self.h,
            w: other.w,
            val: vec![0u8.into(); self.h * other.w].into_boxed_slice(),
        };
        for (res_r, self_r) in res.iter_mut().zip(self.iter()) {
            for (self_val, other_r) in self_r.iter().zip(other.iter()) {
                for (res_val, other_val) in res_r.iter_mut().zip(other_r.iter()) {
                    *res_val = *res_val + *self_val * *other_val;
                }
            }
        }
        res
    }
    pub fn pow(&self, mut k: u64) -> Self {
        assert_eq!(self.h, self.w);
        let n = self.h;
        let mut res = Self::e(n);
        let mut x = self.clone();
        while k > 0 {
            if k & 1 == 1 {
                res = res.mul(&x);
            }
            x = x.mul(&x);
            k >>= 1;
        }
        res
    }
    pub fn pow_assign(&mut self, mut k: usize) {
        use std::mem::swap;
        assert_eq!(self.h, self.w);
        let n = self.h;
        let mut res = Self::e(n);
        while k > 0 {
            if k & 1 == 1 {
                res = res.mul(&self);
            }
            *self = self.mul(&self);
            k >>= 1;
        }
        swap(&mut self.val, &mut res.val);
    }
}
impl<
        T: Copy
            + Eq
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Neg<Output = T>
            + From<u8>,
    > Matrix<T>
{
    pub fn determinant(&self) -> T {
        assert_eq!(self.h, self.w);
        let n = self.h;
        let mut val = self.val.clone();
        let mut res: T = 1.into();
        for i in (0..n).rev() {
            if let Some(k) = val
                .chunks_exact(n)
                .take(i + 1)
                .rposition(|v| v[i] != 0u8.into())
            {
                let (upper, lower) = val.split_at_mut(i * n);
                if k != i {
                    res = -res;
                    upper[k * n..(k + 1) * n].swap_with_slice(&mut lower[..n]);
                }
                res = res * lower[i];
                let inv = <u8 as Into<T>>::into(1u8) / lower[i];
                for lower in lower[..i].iter_mut() {
                    *lower = *lower * inv;
                }
                for upper in upper.chunks_exact_mut(n) {
                    let p = upper[i];
                    for (upper, lower) in upper.iter_mut().zip(lower.iter()) {
                        *upper = *upper - p * *lower;
                    }
                }
            } else {
                return 0.into();
            }
        }
        res
    }
    pub fn gaussian_elimination(&mut self) -> usize {
        let h = self.h;
        let w = self.w;
        let mut x = 0;
        let mut tmp = Vec::with_capacity(w);
        for y in 0..w {
            if let Some(k) = (x..h).find(|k| self[*k][y] != 0u8.into()) {
                for j in 0..w {
                    self.val.swap(x * w + j, k * w + j);
                }
                let inv = <u8 as Into<T>>::into(1u8) / self[x][y];
                for v in self[x].iter_mut().skip(y) {
                    *v = *v * inv;
                }
                tmp.clear();
                tmp.extend_from_slice(&self[x]);
                for (i, r) in self.iter_mut().enumerate() {
                    if i == x {
                        continue;
                    }
                    let p = r[y];
                    for (v, tmp) in r[y..].iter_mut().zip(tmp[y..].iter()) {
                        *v = *v - p * *tmp;
                    }
                }
                x += 1;
            }
        }
        x
    }
}
impl<T: Copy> std::ops::Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.val[index * self.w..(index + 1) * self.w]
    }
}
impl<T: Copy> std::ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.val[index * self.w..(index + 1) * self.w]
    }
}
