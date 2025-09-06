/// Dijstra algorithm.
/// This function has a time complexity O(n log m) in build.
pub struct Dijkstra {
    n: usize,
    source: usize,
    dist: Vec<u64>,
    prev: Vec<usize>,
}
impl Dijkstra {
    pub const INF: u64 = !0;
    /// Build a dijsktra struct.
    /// This function has a time complexity of O((n + m) log n).
    pub fn build(source: usize, n: usize, e: &[(usize, usize, u64)], is_directed: bool) -> Self {
        assert!(source < n);
        use std::collections::BinaryHeap;
        let mut g = vec![vec![]; n];
        for &(i, j, d) in e {
            g[i].push((j, d));
            if !is_directed {
                g[j].push((i, d));
            }
        }
        let mut dist = vec![Self::INF; n];
        let mut prev = vec![source; n];
        let mut heap = BinaryHeap::new();
        dist[source] = 0;
        heap.push((!0, source));
        while let Some((d, i)) = heap.pop() {
            let d = !d;
            if dist[i] < d {
                continue;
            }
            for &(j, dd) in &g[i] {
                if d + dd < dist[j] {
                    dist[j] = d + dd;
                    prev[j] = i;
                    heap.push((!(d + dd), j));
                }
            }
        }
        Dijkstra {
            n,
            source,
            dist,
            prev,
        }
    }
    /// Return the shortest distance between source and sink.
    /// This functions has a time complexity of O(1)
    pub fn dist(&self, sink: usize) -> Option<u64> {
        assert!(sink < self.n);
        if self.dist[sink] == Self::INF {
            None
        } else {
            Some(self.dist[sink])
        }
    }
    /// Return the shortest path between source and sink.
    /// This function has a time complexity of O(n).
    pub fn path(&self, sink: usize) -> Option<(u64, Vec<usize>)> {
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
