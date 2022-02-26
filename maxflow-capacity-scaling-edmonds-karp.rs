// https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=6358043
fn main() {
  input! {
    n: usize, m: usize,
    arcs: [(usize, usize, u64); m],
  }
  let s = 0;
  let t = n - 1;

  let mut graph = MaxFlowCapacityScalingEdmondsKarp::new(n);
  for &(u, v, c) in &arcs {
    graph.add_arc(u, v, c);
  }

  println!("{}", graph.flow(s, t));
}

// ここから

pub struct MaxFlowCapacityScalingEdmondsKarp {
  n: usize,
  arcs: Vec<Arc>,
  graph: Vec<Vec<usize>>,
  max_cap: u64,
}

pub struct Arc {
  pub to: usize,
  pub rev: usize,
  pub cap: u64,
  pub flow: u64,
}

impl MaxFlowCapacityScalingEdmondsKarp {
  pub fn new(n: usize) -> Self {
    Self {
      n,
      arcs: Vec::new(),
      graph: vec![vec![]; n],
      max_cap: 0,
    }
  }

  pub fn add_arc(&mut self, from: usize, to: usize, cap: u64) -> usize {
    assert!(from < self.n && to < self.n);
    self.max_cap = self.max_cap.max(cap);
    let id = self.arcs.len();
    self.arcs.push(Arc { to, rev: id + 1, cap, flow: 0 });
    self.arcs.push(Arc { to: from, rev: id, cap, flow: cap });
    self.graph[from].push(id);
    self.graph[to].push(id + 1);
    id
  }

  pub fn get_arc(&self, id: usize) -> &Arc {
    assert!(id < self.arcs.len());
    &self.arcs[id]
  }

  pub fn flow(&mut self, source: usize, sink: usize) -> u64 {
    self.flow_limited(source, sink, std::u64::MAX)
  }

  pub fn flow_limited(&mut self, source: usize, sink: usize, limit: u64) -> u64 {
    if limit == 0 {
      return 0;
    }
    let mut flow_sum = 0;
    let mut flow = self.max_cap.next_power_of_two();
    while flow > 0 {
      loop {
        let mut visited = vec![false; self.n];
        let mut prev = vec![0; self.n];
        let mut queue = std::collections::VecDeque::new();
        visited[source] = true;
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
          for &arc_id in &self.graph[u] {
            let arc = &mut self.arcs[arc_id];
            if visited[arc.to] || arc.cap - arc.flow < flow {
              continue;
            }
            visited[arc.to] = true;
            prev[arc.to] = arc_id;
            queue.push_back(arc.to);
          }
        }
        if !visited[sink] {
          break;
        }
        let mut to = sink;
        while to != source {
          let (forward, backward) = self.arc_pair(prev[to]);
          forward.flow += flow;
          backward.flow -= flow;
          to = backward.to;
        }
        flow_sum += flow;
      }
      flow >>= 1;
    }
    flow_sum
  }

  fn arc_pair(&mut self, arc_id: usize) -> (&mut Arc, &mut Arc) {
    let rev_id = self.arcs[arc_id].rev;
    if arc_id < rev_id {
      let (head, tail) = self.arcs.split_at_mut(rev_id);
      (&mut head[arc_id], &mut tail[0])
    } else {
      let (head, tail) = self.arcs.split_at_mut(arc_id);
      (&mut tail[0], &mut head[rev_id])
    }
  }
}

// ここまで

#[macro_export]
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}
