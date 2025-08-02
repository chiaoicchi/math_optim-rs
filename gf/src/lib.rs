pub mod combinatorics;
/// galois field Z/MOD Z.
///
/// # Example
///
/// ```
/// ```
///
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GF<const MOD: u32> {
    val: u32,
}
impl<const MOD: u32> GF<MOD> {
    /// This is initializer of `GF`.
    /// This function has a time complexity of O(1).
    pub fn new(val: u32) -> Self {
        Self { val: val % MOD }
    }
    /// Return `val.pow(exp) % MOD`.
    /// This function has a time complexity of O(log exp).
    pub fn pow(&self, mut exp: u32) -> Self {
        let mut res = Self::new(1);
        let mut base = *self;
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base *= base;
            exp >>= 1;
        }
        res
    }
    /// Return `x` such that `self * x == x * self == 1`.
    /// This function has a time complexity of O(log MOD).
    pub fn inv(&self) -> Self {
        assert_ne!(self.val, 0);
        self.pow(MOD - 2)
    }
}
use std::fmt::{Debug, Display, Formatter, Result};
impl<const MOD: u32> Debug for GF<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.val)
    }
}
impl<const MOD: u32> Display for GF<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.val)
    }
}
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
impl<const MOD: u32> Neg for GF<MOD> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        if self.val > 0 {
            self.val = MOD - self.val;
        }
        self
    }
}
impl<const MOD: u32> AddAssign<GF<MOD>> for GF<MOD> {
    fn add_assign(&mut self, rhs: GF<MOD>) {
        self.val += rhs.val;
        if self.val >= MOD {
            self.val -= MOD;
        }
    }
}
impl<const MOD: u32> SubAssign<GF<MOD>> for GF<MOD> {
    fn sub_assign(&mut self, rhs: GF<MOD>) {
        if self.val < rhs.val {
            self.val += MOD;
        }
        self.val -= rhs.val;
    }
}
impl<const MOD: u32> MulAssign<GF<MOD>> for GF<MOD> {
    fn mul_assign(&mut self, rhs: GF<MOD>) {
        self.val = ((self.val as u64 * rhs.val as u64) % MOD as u64) as u32;
    }
}
impl<const MOD: u32> DivAssign<GF<MOD>> for GF<MOD> {
    fn div_assign(&mut self, rhs: GF<MOD>) {
        self.val = ((self.val as u64 * rhs.inv().val as u64) % MOD as u64) as u32;
    }
}
macro_rules! gf_ops {
    ($(
            $trait:ident,
            $trait_assign:ident,
            $fn:ident,
            $fn_assign:ident,
    )*) => {$(
        impl<const MOD: u32> $trait_assign<&GF<MOD>> for GF<MOD> {
            fn $fn_assign(&mut self, rhs: &GF<MOD>) {
                self.$fn_assign(*rhs);
            }
        }
        impl<const MOD: u32, T: Into<GF<MOD>>> $trait<T> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl<const MOD: u32> $trait<&GF<MOD>> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<const MOD: u32, T: Into<GF<MOD>>> $trait<T> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl<const MOD: u32> $trait<&GF<MOD>> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
                (*self).$fn(*rhs)
            }
        }
    )*};
}
gf_ops! {
    Add, AddAssign, add, add_assign,
    Sub, SubAssign, sub, sub_assign,
    Mul, MulAssign, mul, mul_assign,
    Div, DivAssign, div, div_assign,
}
use std::iter::{Product, Sum};
impl<const MOD: u32> Sum for GF<MOD> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |acc, a| acc + a)
    }
}
impl<'a, const MOD: u32> Sum<&'a Self> for GF<MOD> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}
impl<const MOD: u32> Product for GF<MOD> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |acc, a| acc * a)
    }
}
impl<'a, const MOD: u32> Product<&'a Self> for GF<MOD> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}
#[macro_export]
macro_rules! gf {
    ($value:expr) => {
        $crate::GF::from($value)
    };
    ($value:expr, $p:expr) => {
        $crate::GF::<$p>::from($value)
    };
}
macro_rules! gf_new_from_signed {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u32> From<$t> for GF<MOD> {
                fn from(x: $t) -> Self {
                    if x < 0 {
                        - Self::new((MOD as i64 - x as i64) as u32)
                    } else {
                        Self::new(x as u32)
                    }
                }
            }
        )*
    };
}
gf_new_from_signed!(i8, i16, i32, i64, i128, isize);
macro_rules! gf_new_from_unsigned {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u32> From<$t> for GF<MOD> {
                fn from(x: $t) -> Self {
                    Self::new(x as u32)
                }
            }
        )*
    };
}
gf_new_from_unsigned!(u8, u16, u32, u64, u128, usize);
