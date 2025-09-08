pub mod euler_tour;
pub mod lca;
/// Return tree diameter.
/// This function has a time complexity of O(n).
pub fn diameter(n: usize, e: &[(usize, usize, u64)]) -> (u64, Vec<usize>) {
    use std::iter::{once, successors};
    assert_eq!(n, e.len() + 1);
    let mut g = vec![vec![]; n];
    e.iter().for_each(|&(u, v, d)| {
        g[u].push((v, d));
        g[v].push((u, d));
    });
    let mut dist = vec![!0; n];
    dist[0] = 0;
    let mut stack = vec![0];
    while let Some(i) = stack.pop() {
        for &(j, d) in &g[i] {
            if dist[j] == !0 {
                dist[j] = dist[i] + d;
                stack.push(j);
            }
        }
    }
    let (mut p, mut mx) = (0, 0);
    for (i, &d) in dist.iter().enumerate() {
        if d > mx {
            p = i;
            mx = d;
        }
    }
    dist.fill(!0);
    stack.push(p);
    dist[p] = 0;
    let mut prev = vec![p; n];
    while let Some(i) = stack.pop() {
        for &(j, d) in &g[i] {
            if dist[j] == !0 {
                dist[j] = dist[i] + d;
                prev[j] = i;
                stack.push(j);
            }
        }
    }
    let mx = dist.iter().filter(|d| **d != 0).max().unwrap();
    let q = dist.iter().position(|d| d == mx).unwrap();
    let path = successors(Some(q), |i| Some(prev[*i]))
        .take_while(|i| *i != p)
        .chain(once(p))
        .collect::<Vec<_>>();
    (dist[q], path)
}
