/// Return run length encoding of `a`.
/// This function has a time complexity of O(n).
pub fn rle<T: Copy + PartialEq>(a: &[T]) -> Vec<(T, usize)> {
    let mut a = a.iter().copied().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}
/// Return run length decoding of run length encoded array.
/// This function has a time complexity of length of decoded array.
pub fn rld<T: Copy + PartialEq>(a: &[(T, usize)]) -> Vec<T> {
    a.iter()
        .map(|a| std::iter::repeat(a.0).take(a.1))
        .flatten()
        .collect()
}
