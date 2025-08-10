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
