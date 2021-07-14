// Verify: https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=5687544
pub mod maxflow_push_relabel {
    // https://codeforces.com/blog/entry/58048?#comment-417656
    use std::collections::*;
    use std::ops::*;

    #[derive(Debug, Clone, Default)]
    pub struct MFGraph<F> {
        n: usize,
        edges: Vec<Edge<F>>,
        graph: Vec<Vec<usize>>,
        h: Vec<usize>,
        inq: Vec<bool>,
        count: Vec<usize>,
        excess: Vec<F>,
        q: VecDeque<usize>,
    }
    impl<F: Copy + Default + Add<Output = F> + Sub<Output = F> + PartialEq + Ord> MFGraph<F> {
        /// `n` 頂点の最大流グラフを作成する
        pub fn new(n: usize) -> Self {
            Self {
                n,
                edges: vec![],
                graph: vec![vec![]; n],
                h: vec![0; n],
                inq: vec![false; n],
                count: vec![0; n * 2],
                excess: vec![F::default(); n],
                q: VecDeque::new(),
            }
        }

        /// 容量 `cap` の `from` から `to` への有向辺を追加する
        pub fn add_edge(&mut self, from: usize, to: usize, cap: F) -> usize {
            let e = self.edges.len();
            self.edges.push(Edge {
                from,
                to,
                cap,
                ..Edge::default()
            });
            self.graph[from].push(e);
            self.edges.push(Edge {
                from: to,
                to: from,
                ..Edge::default()
            });
            self.graph[to].push(e + 1);
            e
        }

        pub fn edge(&self, e: usize) -> &Edge<F> {
            &self.edges[e]
        }

        fn edge_mut(&mut self, e: usize) -> &mut Edge<F> {
            &mut self.edges[e]
        }

        fn enqueue(&mut self, u: usize) {
            if !self.inq[u] && self.excess[u] != F::default() {
                self.q.push_back(u);
                self.inq[u] = true;
            }
        }

        fn push(&mut self, e: usize) {
            let from = self.edge(e).from;
            let to = self.edge(e).to;
            let to_push = self.excess[from].min(self.edge(e).cap - self.edge(e).flow);
            if to_push != F::default() && self.h[from] > self.h[to] {
                self.edge_mut(e).flow = self.edge(e).flow + to_push;
                self.excess[to] = self.excess[to] + to_push;
                self.excess[from] = self.excess[from] - to_push;
                self.edge_mut(e ^ 1).flow = self.edge(e ^ 1).flow - to_push;
                self.enqueue(to);
            }
        }

        fn relabel(&mut self, u: usize) {
            self.count[self.h[u]] -= 1;
            self.h[u] = 2 * self.n - 2;
            for &e in &self.graph[u] {
                if self.edge(e).cap > self.edge(e).flow {
                    self.h[u] = self.h[u].min(self.h[self.edge(e).to]);
                }
            }
            self.h[u] += 1;
            self.count[self.h[u]] += 1;
        }

        fn gap_relabel(&mut self, height: usize) {
            for u in 0..self.n {
                if self.h[u] >= height && self.h[u] < self.n {
                    self.count[self.h[u]] -= 1;
                    self.count[self.n] += 1;
                    self.h[u] = self.n;
                    self.enqueue(u)
                }
            }
        }

        fn discharge(&mut self, u: usize) {
            for i in 0..self.graph[u].len() {
                if self.excess[u] != F::default() {
                    self.push(self.graph[u][i]);
                } else {
                    break;
                }
            }
            if self.excess[u] != F::default() {
                if self.h[u] < self.n && self.count[self.h[u]] < 2 {
                    self.gap_relabel(self.h[u]);
                } else {
                    self.relabel(u);
                }
            } else if let Some(_) = self.q.pop_front() {
                self.inq[u] = false;
            }
        }

        /// `s` から `t` への最大流を求める
        pub fn flow(&mut self, s: usize, t: usize) -> F {
            self.h[s] = self.n;
            self.inq[s] = true;
            self.inq[t] = true;
            self.count[0] = self.n - 1;
            self.count[self.n] = 1;
            for i in 0..self.graph[s].len() {
                let e = self.graph[s][i];
                self.excess[s] = self.excess[s] + self.edge(e).cap;
                self.push(e);
            }
            while let Some(&u) = self.q.front() {
                self.discharge(u);
            }
            self.excess[t]
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct Edge<F> {
        from: usize,
        to: usize,
        cap: F,
        flow: F,
    }
}
