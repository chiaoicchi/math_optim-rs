pub mod rhash;
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
/// Return strictly longest increasing subsequence size and its example.
/// This function has time complexity of O(n log n).
pub fn lis<T: Copy + PartialOrd>(a: &[T], inf: T) -> (usize, Vec<usize>) {
    let n = a.len();
    let mut dp = vec![inf; n + 1];
    let mut f = vec![];
    a.iter().for_each(|a| {
        let pos = dp.partition_point(|dp| dp < a);
        dp[pos] = *a;
        f.push(pos);
    });
    let mut lis = vec![];
    let mut pos = dp.iter().rposition(|dp| *dp < inf).unwrap();
    for (i, f) in f.iter().enumerate().rev() {
        if *f == pos {
            lis.push(i);
            if pos == 0 {
                break;
            } else {
                pos -= 1;
            }
        }
    }
    lis.reverse();
    (lis.len(), lis)
}
/// Return compressed of sequence.
/// This function has time complexity of O(n log n).
pub fn compressed<T: Copy + PartialOrd>(a: &[T]) -> Vec<usize> {
    let n = a.len();
    let mut sorted = a.to_vec();
    sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    sorted.dedup();
    let mut compressed = Vec::with_capacity(n);
    a.iter()
        .for_each(|a| compressed.push(sorted.partition_point(|x| x < a)));
    compressed
}
/// Return next index vector.
/// res[pos][c] := index which is greater than or equal to pos s.t. c
/// If there is no result, return |s|.
/// Must be l <= s < r.
/// This function has time complexity of O((r - l)|s|).
pub fn next_pos(s: &[u8], l: u8, r: u8) -> Vec<Vec<usize>> {
    let n = s.len();
    let mut res = Vec::with_capacity(n);
    let mut dp = vec![n; (r - l) as usize];
    let mut tmp = vec![0; (r - l) as usize];
    for (i, &c) in s.iter().enumerate().rev() {
        dp[(c - l) as usize] = i;
        tmp.copy_from_slice(&dp);
        res.push(tmp.clone());
    }
    res.reverse();
    res
}
