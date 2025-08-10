pub mod ntt;
/// This is discrete convolution.
///
/// # Example
///
/// ```
/// ```
///
pub trait DConv {
    type S;
    fn ft(&mut self);
    fn ift(&mut self);
    fn conv(&self, rhs: &[Self::S]) -> Vec<Self::S>;
}
