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
    let mut p = 0;
    while let Some(i) = stack.pop() {
        g[i].iter().for_each(|&(j, d)| {
            if dist[j] == !0 {
                dist[j] = dist[i] + d;
                if dist[p] < dist[j] {
                    p = j;
                }
                stack.push(j);
            }
        });
    }
    dist.fill(!0);
    stack.push(p);
    dist[p] = 0;
    let mut prev = vec![p; n];
    let mut q = p;
    while let Some(i) = stack.pop() {
        g[i].iter().for_each(|&(j, d)| {
            if dist[j] == !0 {
                dist[j] = dist[i] + d;
                prev[j] = i;
                if dist[q] < dist[j] {
                    q = j;
                }
                stack.push(j);
            }
        });
    }
    let path = successors(Some(q), |i| Some(prev[*i]))
        .take_while(|i| *i != p)
        .chain(once(p))
        .collect::<Vec<_>>();
    (dist[q], path)
}

/// Return strongly connected components.
/// This function has a time complexity of O(n + m).
pub fn kasaraju(n: usize, e: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut g = vec![vec![]; n];
    let mut ig = vec![vec![]; n];
    e.iter().for_each(|&(u, v)| {
        g[u].push(v);
        ig[v].push(u);
    });
    let mut ord = Vec::with_capacity(n);
    let mut used = vec![false; n];
    (0..n).for_each(|i| {
        if !used[i] {
            _dfs(i, &g, &mut used, &mut ord);
        }
    });
    let mut scc = vec![];
    used.fill(false);
    ord.iter().rev().for_each(|&i| {
        if !used[i] {
            let mut sc = vec![];
            _efs(i, &ig, &mut used, &mut sc);
            scc.push(sc);
        }
    });
    scc
}
fn _dfs(i: usize, g: &[Vec<usize>], used: &mut [bool], ord: &mut Vec<usize>) {
    used[i] = true;
    g[i].iter().for_each(|&j| {
        if !used[j] {
            _dfs(j, g, used, ord);
        }
    });
    ord.push(i);
}
fn _efs(i: usize, ig: &[Vec<usize>], used: &mut [bool], sc: &mut Vec<usize>) {
    used[i] = true;
    sc.push(i);
    ig[i].iter().for_each(|&j| {
        if !used[j] {
            _efs(j, ig, used, sc);
        }
    })
}
