pub struct EulerTour {
    parent: Vec<usize>,
    in_time: Vec<usize>,
    out_time: Vec<usize>,
    euler_tour: Vec<isize>,
}
impl EulerTour {
    pub fn build(source: usize, n: usize, e: &[(usize, usize)]) -> Self {
        assert_eq!(n, e.len() + 1);
        let mut g = vec![vec![]; n];
        for &(i, j) in e {
            g[i].push(j);
            g[j].push(i);
        }
        let mut parent = vec![n; n];
        let mut stack = vec![source];
        let mut in_time = vec![!0; n];
        let mut out_time = vec![!0; n];
        let mut euler_tour = Vec::with_capacity(2 * n);
        let mut t = 0;
        while let Some(i) = stack.pop() {
            if in_time[i] == !0 {
                euler_tour.push(i as isize);
                in_time[i] = t;
                t += 1;
                stack.push(i);
                for &j in &g[i] {
                    if in_time[j] == !0 {
                        parent[j] = i;
                        stack.push(j);
                    }
                }
            } else {
                out_time[i] = t;
                euler_tour.push(-1 * i as isize);
                t += 1;
            }
        }
        Self {
            parent,
            in_time,
            out_time,
            euler_tour,
        }
    }
    pub fn in_time(&self, i: usize) -> usize {
        self.in_time[i]
    }
    pub fn out_time(&self, i: usize) -> usize {
        self.out_time[i]
    }
    pub fn euler_tour(&self) -> &Vec<isize> {
        &self.euler_tour
    }
}
