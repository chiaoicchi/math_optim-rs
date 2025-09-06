/// Compare 2 points with its argment.
/// This function has a time complexity of O(1)
pub fn arg_cmp((x0, y0): &(i64, i64), (x1, y1): &(i64, i64)) -> std::cmp::Ordering {
    ((*y0, *x0) < (0, 0))
        .cmp(&((*y1, *x1) < (0, 0)))
        .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
}
