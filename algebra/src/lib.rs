/// monoid trait.
///
/// # Example
///
/// ```
/// ```
///
pub trait Monoid {
    type S: Clone;
    const E: Self::S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}
pub trait ComMonoid: Monoid {}
/// group trait.
///
/// # Example
///
/// ```
/// ```
///
pub trait Group {
    type S: Clone + PartialEq;
    const E: Self::S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
    fn inv(val: &Self::S) -> Self::S;
}
/// abelian trait.
///
/// # Example
///
/// ```
/// ```
///
pub trait Abelian: Group {}
