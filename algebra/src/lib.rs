/// monoid trait.
///
/// # Example
///
/// ```
/// ```
///
pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}
/// group trait.
///
/// # Example
///
/// ```
/// ```
///
pub trait Group {
    type S: Clone + PartialEq;
    fn e() -> Self::S;
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
/// ring trait.
///
/// # Example
///
/// ```
/// ```
///
pub trait Ring: Abelian + Monoid {}
/// commutative ring trait.
///
/// # Example
///
/// ```
/// ```
///
pub trait CR: Ring {}
/// field trait.
///
/// # Example
///
/// ```
/// ```
///
pub trait Field: Abelian + Group {}
