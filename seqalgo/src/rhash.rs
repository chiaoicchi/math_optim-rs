/// This is rolling hash.
///
/// # Example
///
/// ```
/// ```
///
pub const BASE: u64 = 1_025;
pub const HASH: u64 = (1 << 61) - 1;
fn _mulu128(a: &u64, b: &u64) -> u64 {
    let t = *a as u128 * *b as u128;
    let u = (t >> 61) as u64 + (t & HASH as u128) as u64;
    if u >= HASH {
        u - HASH
    } else {
        u
    }
}
pub struct RHash<const BASE: u64, const HASH: u64> {
    n: usize,
    hash_acc: Vec<u64>,
    base_pow: Vec<u64>,
}
impl<const BASE: u64, const HASH: u64> RHash<BASE, HASH> {
    /// Return hash of range.
    /// This function has a time complexity of O(1).
    pub fn range_hash(&self, range: impl std::ops::RangeBounds<usize>) -> u64 {
        use std::ops::Bound::{Excluded, Included, Unbounded};
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(x) => *x,
            Excluded(x) => x + 1,
        };
        let r = match range.end_bound() {
            Unbounded => self.n,
            Included(x) => x + 1,
            Excluded(x) => *x,
        };
        assert!(r <= self.n);
        assert!(l <= r);
        let left = _mulu128(&self.hash_acc[l], &self.base_pow[r - l]);
        let right = self.hash_acc[r];
        if right < left {
            HASH - left + right
        } else {
            right - left
        }
    }
}
/// Constructor from iter to RollingHash.
/// This function has a time complexity of O(n).
use std::iter::FromIterator;
macro_rules! rhash_new {
    ($($t:ty), *) => {
        $(
            impl<const BASE: u64, const HASH: u64> FromIterator<$t> for RHash<BASE, HASH> {
                #[inline]
                fn from_iter<I: IntoIterator<Item = $t>>(iter: I) -> RHash<BASE, HASH> {
                    let mut hash_acc = vec![0];
                    let mut base_pow = vec![1];
                    iter.into_iter().for_each(|a| {
                        let mut h = _mulu128(hash_acc.last().unwrap(), &BASE);
                        h += a as u64 + 1;
                        if h >= HASH {
                            h -= HASH;
                        }
                        hash_acc.push(h);
                        let b = _mulu128(base_pow.last().unwrap(), &BASE);
                        base_pow.push(b);
                    });
                    Self {
                        n: hash_acc.len() - 1,
                        hash_acc,
                        base_pow,
                    }
                }
            }
        )*
    }
}
macro_rules! rhash_new_from_ref {
    ($($t:ty), *) => {
        $(
            impl<'a, const BASE: u64, const HASH: u64> FromIterator<$t> for RHash<BASE, HASH> {
                #[inline]
                fn from_iter<I: IntoIterator<Item = $t>>(iter: I) -> RHash<BASE, HASH> {
                    let mut hash_acc = vec![0];
                    let mut base_pow = vec![1];
                    iter.into_iter().for_each(|a| {
                        let mut h = _mulu128(hash_acc.last().unwrap(), &BASE);
                        h += *a as u64 + 1;
                        if h >= HASH {
                            h -= HASH;
                        }
                        hash_acc.push(h);
                        let b = _mulu128(base_pow.last().unwrap(), &BASE);
                        base_pow.push(b);
                    });
                    Self {
                        n: hash_acc.len() - 1,
                        hash_acc,
                        base_pow,
                    }
                }
            }
        )*
    }
}
rhash_new!(u8, u16, u32, u64, u128);
rhash_new_from_ref!(&'a u8, &'a u16, &'a u32, &'a u64, &'a u128);
