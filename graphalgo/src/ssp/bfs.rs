/// BFS algorithm.
/// This function has a time complexity O(n log m) in build.
pub struct BFS {
    n: usize,
    source: usize,
    dist: Vec<usize>,
    prev: Vec<usize>,
}
impl BFS {
    pub const INF: usize = !0;
    /// Build a dijsktra struct.
    /// This function has a time complexity of O(n + m).
    pub fn build(source: usize, n: usize, e: &[(usize, usize)], is_directed: bool) -> Self {
        assert!(source < n);
        use std::collections::VecDeque;
        let mut g = vec![vec![]; n];
        for &(i, j) in e {
            g[i].push(j);
            if !is_directed {
                g[j].push(i);
            }
        }
        let mut dist = vec![Self::INF; n];
        let mut prev = vec![source; n];
        let mut que = VecDeque::new();
        dist[source] = 0;
        que.push_back(source);
        while let Some(i) = que.pop_front() {
            for &j in &g[i] {
                if dist[j] == Self::INF {
                    dist[j] = dist[i] + 1;
                    prev[j] = i;
                    que.push_back(j);
                }
            }
        }
        BFS {
            n,
            source,
            dist,
            prev,
        }
    }
    /// Return the shortest distance between source and sink.
    /// This functions has a time complexity of O(1)
    pub fn dist(&self, sink: usize) -> Option<usize> {
        assert!(sink < self.n);
        if self.dist[sink] == Self::INF {
            None
        } else {
            Some(self.dist[sink])
        }
    }
    /// Return the shortest path between source and sink.
    /// This function has a time complexity of O(n).
    pub fn path(&self, sink: usize) -> Option<(usize, Vec<usize>)> {
        assert!(sink < self.n);
        if self.dist[sink] == Self::INF {
            None
        } else {
            let mut path = vec![sink];
            let mut tmp = sink;
            while tmp != self.source {
                tmp = self.prev[tmp];
                path.push(tmp);
            }
            path.reverse();
            Some((self.dist[sink], path))
        }
    }
}
