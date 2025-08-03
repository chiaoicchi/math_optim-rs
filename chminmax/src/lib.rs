/// This is Change Min Max trait.
pub trait ChMinMax {
    fn chmin(&mut self, x: Self) -> bool;
    fn chmax(&mut self, x: Self) -> bool;
}
impl<T: PartialOrd> ChMinMax for T {
    /// Return whether `self > x` , and if that change it to `x`.
    fn chmin(&mut self, x: Self) -> bool {
        *self > x && {
            *self = x;
            true
        }
    }
    /// Return wherher `self < x`, and if that change it to `x`.
    fn chmax(&mut self, x: Self) -> bool {
        *self < x && {
            *self = x;
            true
        }
    }
}
