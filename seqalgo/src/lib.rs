pub mod rle;
/// Return inversions of `a`.
/// This function has time complexity of O(log n).
pub fn inversions<T: Copy + PartialOrd>(a: &[T]) -> usize {
    use std::iter::successors;
    if a.len() == 0 {
        0
    } else {
        let n = a.len();
        let mut sorted = a.to_vec();
        sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        sorted.dedup();
        let mut compressed = Vec::with_capacity(n);
        a.iter()
            .for_each(|a| compressed.push(sorted.partition_point(|x| x < a)));
        let m = compressed.iter().max().unwrap() + 1;
        let mut bitree = vec![0; m + 1];
        let mut res = 0;
        compressed.iter().enumerate().for_each(|(i, x)| {
            res += i;
            successors(Some(x + 1), |x| Some(x - (x & x.wrapping_neg())))
                .take_while(|x| *x > 0)
                .for_each(|x| res -= bitree[x]);
            successors(Some(x + 1), |x| Some(x + (x & x.wrapping_neg())))
                .take_while(|x| *x <= m)
                .for_each(|x| bitree[x] += 1);
        });
        res
    }
}
