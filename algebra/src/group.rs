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
