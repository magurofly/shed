#![allow(non_snake_case)]

// #[fastout]
fn main() {
  let mut graph = MFGraph::<i64>::with_vertices(5);
  let edges = vec![(0, 1, 3), (0, 2, 3), (1, 2, 2), (1, 3, 1), (2, 3, 3), (2, 4, 2), (3, 4, 2)];
  println!("{:?}", &edges);
  graph.add_edges(edges);
  println!("{:?}", graph.shortest_path_bfs::<usize>(4));
  // let flow = graph.maxflow_push_relabel(0, 4);
  // println!("{}", flow);
}

// use proconio::{input, fastout, marker::Usize1};
use self::graphs::*;

pub mod graphs {
  use std::marker::PhantomData;
  use std::collections::*;
  use std::cmp::*;
  use num_traits::*;

  pub fn zip<T, U>(left: Option<T>, right: Option<U>) -> Option<(T, U)> {
    left.and_then(|x| right.map(|y| (x, y) ))
  }
  
  pub trait AssignOps: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign {}
  pub trait Measure: std::fmt::Debug + Num + Default + Ord + Copy + AssignOps + std::iter::Sum {
    fn chmin(&mut self, other: Self) -> &mut Self {
      if *self > other {
        *self = other;
      }
      self
    }
    fn chmax(&mut self, other: Self) -> &mut Self {
      if *self < other {
        *self = other;
      }
      self
    }
    fn if_chmin<F: FnOnce()>(&mut self, other: Self, procedure: F) -> &mut Self {
      if *self > other {
        *self = other;
        (procedure)();
      }
      self
    }
    fn if_chmax<F: FnOnce()>(&mut self, other: Self, procedure: F) -> &mut Self {
      if *self < other {
        *self = other;
        (procedure)();
      }
      self
    }
  }
  pub trait MeasureSigned: Measure + Signed {}

  #[derive(Debug, Default, Clone)]
  pub struct Vertex<V> {
    id: usize,
    weight: V,
    edges: Vec<usize>,
  }

  #[derive(Debug, Default, Clone)]
  pub struct Edge<E> {
    id: usize,
    weight: E,
    from: usize,
    to: usize,
    rev: Option<usize>,
  }

  pub trait EdgeType<E, D> {
    fn add_edge<V>(graph: &mut VecGraph<V, E, D>, from: usize, to: usize, weight: E) -> usize;
  }
  /// 有向グラフ
  #[derive(Debug)]
  pub enum Directed {}
  impl<E> EdgeType<E, Directed> for Directed {
    fn add_edge<V>(graph: &mut VecGraph<V, E, Directed>, from: usize, to: usize, weight: E) -> usize {
      assert!(from < graph.vertices.len() && to < graph.vertices.len());
      let id = graph.edges.len();
      graph.edges.push(Edge { id, weight, from, to, rev: None });
      graph.vertices[from].edges.push(id);
      id
    }
  }
  /// 無向グラフ
  #[derive(Debug)]
  pub enum Undirected {}
  impl<E: Clone> EdgeType<E, Undirected> for Undirected {
    fn add_edge<V>(graph: &mut VecGraph<V, E, Undirected>, from: usize, to: usize, weight: E) -> usize {
      assert!(from < graph.vertices.len() && to < graph.vertices.len());
      let id = graph.edges.len();
      graph.edges.push(Edge { id, weight: weight.clone(), from, to, rev: Some(id + 1) });
      graph.vertices[from].edges.push(id);
      graph.edges.push(Edge { id: id + 1, weight, from: to, to: from, rev: Some(id) });
      graph.vertices[to].edges.push(id + 1);
      id
    }
  }
  /// 最大フロー（残余グラフ）
  #[derive(Debug)]
  pub enum MaxFlow {}
  impl<Flow: Measure> EdgeType<Flow, MaxFlow> for MaxFlow {
    fn add_edge<V>(graph: &mut VecGraph<V, Flow, MaxFlow>, from: usize, to: usize, capacity: Flow) -> usize {
      assert!(from < graph.vertices.len() && to < graph.vertices.len());
      let id = graph.edges.len();
      graph.edges.push(Edge { id, weight: capacity, from, to, rev: Some(id + 1) });
      graph.vertices[from].edges.push(id);
      graph.edges.push(Edge { id: id + 1, weight: Flow::zero(), from: to, to: from, rev: Some(id) });
      graph.vertices[to].edges.push(id + 1);
      id
    }
  }

  pub type DiGraph<V = (), E = ()> = VecGraph<V, E, Directed>;
  pub type UnGraph<V = (), E = ()> = VecGraph<V, E, Undirected>;
  pub type MFGraph<Flow = ()> = VecGraph<(), Flow, MaxFlow>;

  pub trait IntoVertices<V, I: Iterator<Item = V>> {
    fn into_vertices(self) -> I;
  }
  pub trait IntoEdge<E> {
    fn into_edge(self) -> (usize, usize, E);
  }

  /// グラフのトレイト
  /// メソッドが非常に少ないため、 `VecGraph` からこちらにメソッドを移す予定
  pub trait Graph<V, E> {
    fn vertex(&self, id: usize) -> &Vertex<V>;
    fn edge(&self, id: usize) -> &Edge<E>;
  }

  // TODO: GraphMut 構造体を作成する

  #[derive(Debug, Default)]
  pub struct VecGraph<V = (), E = (), D = Directed> {
    edge_type: PhantomData<D>,
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>,
  }
  impl<V, E, D> Graph<V, E> for VecGraph<V, E, D> {
    fn vertex(&self, id: usize) -> &Vertex<V> {
      assert!(id < self.vertices.len());
      &self.vertices[id]
    }
    
    fn edge(&self, id: usize) -> &Edge<E> {
      assert!(id < self.edges.len());
      &self.edges[id]
    }
  }
  impl<V, E, D: EdgeType<E, D>> VecGraph<V, E, D> {
    /// 空のグラフを生成する
    pub fn new() -> Self {
      Self {
        edge_type: PhantomData,
        vertices: Vec::new(),
        edges: Vec::new(),
      }
    }

    /// 重みのリスト、もしくは頂点の個数からグラフを生成する
    pub fn with_vertices<I: Iterator<Item = V>, Vs: IntoVertices<V, I>>(vertices: Vs) -> Self {
      Self {
        edge_type: PhantomData,
        vertices: vertices.into_vertices().enumerate().map(|(id, weight)| Vertex { id, weight, edges: Vec::new() } ).collect::<Vec<_>>(),
        edges: Vec::new(),
      }
    }

    pub fn add_vertex(&mut self, weight: V) -> usize {
      let id = self.vertices.len();
      self.vertices.push(Vertex { id, weight, edges: Vec::new() });
      id
    }

    /// 頂点の重みのリストまたは頂点数によって頂点を追加し、頂点番号のリストを返す
    pub fn add_vertices<I: Iterator<Item = V>, Vs: IntoVertices<V, I>>(&mut self, vertices: Vs) -> Vec<usize> {
      vertices.into_vertices().map(|v| self.add_vertex(v) ).collect::<Vec<_>>()
    }

    /// 辺の向きを全て反転したグラフ
    pub fn reversed(&self) -> Self where V: Clone, E: Clone {
      let mut graph = Self::new();
      for vertex in &self.vertices {
        graph.add_vertex(vertex.weight.clone());
      }
      for edge in &self.edges {
        graph.add_edge(edge.to, edge.from, edge.weight.clone());
      }
      graph
    }

    pub fn connect(&mut self, from: usize, to: usize) -> usize where E: Clone + Default {
      self.add_edge(from, to, Default::default())
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: E) -> usize where E: Clone {
      D::add_edge(self, from, to, weight)
    }

    /// 辺のリストによって辺を追加する
    /// 要素は `(from, to, weight)` または `(from, to)` となっている必要がある
    pub fn add_edges<I: IntoIterator>(&mut self, edges: I) where E:Clone, I::Item: IntoEdge<E> {
      for edge in edges {
        let (from, to, weight) = edge.into_edge();
        self.add_edge(from, to, weight);
      }
    }

    pub fn vertex_mut(&mut self, id: usize) -> &mut Vertex<V> {
      assert!(id < self.vertices.len());
      &mut self.vertices[id]
    }

    pub fn edge_mut(&mut self, id: usize) -> &mut Edge<E> {
      assert!(id < self.edges.len());
      &mut self.edges[id]
    }

    /// 頂点から出ている辺のリストを返す
    pub fn edges_from(&self, from: usize) -> Vec<usize> {
      self.vertex(from).edges.iter().copied().collect::<Vec<_>>()
    }

    /// 隣接する頂点のリストを返す
    pub fn adjacent_vertices(&self, from: usize) -> Vec<usize> {
      self.vertex(from).edges.iter().map(|&e| self.edge(e).to ).collect::<Vec<_>>()
    }

    /// 最短経路を求める
    /// 負辺がある場合は使えない
    pub fn shortest_path_dijkstra<T, F: FnMut(&Edge<E>) -> Option<T>>(&self, from: usize, mut cost_by: F) -> Vec<Option<T>> where T: Measure {
      assert!(from < self.vertices.len());
      let mut dist = vec![None; self.vertices.len()];
      dist[from] = Some(T::zero());
      let mut pq = BinaryHeap::new();
      pq.push((Reverse(T::zero()), from));
      while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] != Some(d) { continue; }
        for edge in self.vertices[u].edges.iter().map(|&e| self.edge(e) ) {
          if let Some(cost) = (cost_by)(edge) {
            dist[edge.to].if_chmin(d + cost, || pq.push((Reverse(d + cost), edge.to)));
          }
        }
      }
      dist
    }

    /// 全点対最短経路を求める
    pub fn shortest_paths_floyd_warshall<T, F: FnMut(&Edge<E>) -> Option<T>>(&self, make_loop: bool, mut cost_by: F) -> Vec<Vec<Option<T>>> where T: Measure {
      let vertices = self.vertices.len();
      let mut dist = vec![vec![None as Option<T>; vertices]; vertices];
      for edge in &self.edges {
        if let Some(cost) = (cost_by)(edge) {
          dist[edge.from][edge.to].chmin(cost);
        }
      }
      if make_loop {
        for v in 0 .. vertices {
          dist[v][v] = Some(T::zero());
        }
      }
      for k in 0 .. vertices {
        for i in 0 .. vertices {
          for j in 0 .. vertices {
            if let Some((d1, d2)) = zip(dist[i][k], dist[k][j]) {
              dist[i][j].chmin(d1 + d2);
            }
          }
        }
      }
      dist
    }

    // pub fn spfa<T, F: FnMut(&Edge<E>) -> Option<T>>(&self, from: usize, mut cost_by: F) -> Vec<Result<T, Status>>

    /// 最大フローを求める (Generic Push/Relabel + Global Relabeling + FIFO)
    pub fn maxflow_push_relabel<'a>(&'a mut self, s: usize, t: usize) -> E where E: Measure {
      struct State<'b, V, E, D> {
        graph: &'b mut VecGraph<V, E, D>,
        excess: Vec<E>,
        label: Vec<usize>,
        active_vertices: VecDeque<usize>,
        source: usize,
        sink: usize,
      }
      impl<'b, V, E: Measure, D: EdgeType<E, D>> State<'b, V, E, D> {
        fn new(graph: &'b mut VecGraph<V, E, D>, source: usize, sink: usize) -> Self {
          let excess = vec![E::zero(); graph.vertices.len()];
          let label = vec![0; graph.vertices.len()];
          let mut this = Self { graph, source, sink, excess, label, active_vertices: VecDeque::new() };
          for e in this.graph.edges_from(source) {
            this.excess[source] += this.graph.edge(e).weight;
          }
          this
        }

        fn relabel_global(&mut self) {
          for (v, d) in self.graph.shortest_path_bfs(self.sink).into_iter().enumerate() {
            self.label[v] = d.unwrap_or(0);
          }
          self.label[self.source] = self.graph.vertices.len();
        }

        fn push(&mut self, from: usize) -> bool {
          let mut admissible_edges = false;
          for e in self.graph.edges_from(from) {
            let to = self.graph.edge(e).to;
            if !(self.label[from] >= self.label[to] + 1) { continue; }
            admissible_edges = true;
            let preflow = self.graph.edge(e).weight.min(self.excess[from]);
            self.excess[from] -= preflow;
            self.excess[to] += preflow;
            self.graph.edge_mut(e).weight -= preflow;
            self.graph.edge_mut(self.graph.edge(e).rev.unwrap()).weight += preflow;
            if to != self.sink { self.active_vertices.push_back(to); }
          }
          admissible_edges
        }

        fn relabel(&mut self, from: usize) {
          self.graph.adjacent_vertices(from).into_iter().map(|v| self.label[v] ).max().unwrap_or(self.graph.vertices.len());
        }
      }

      let mut state = State::new(self, s, t);
      state.relabel_global();

      eprintln!("excess: {:?}", &state.excess);
      eprintln!("label: {:?}", &state.label);

      state.push(s);

      eprintln!("excess: {:?}", &state.excess);
      eprintln!("label: {:?}", &state.label);
      
      while let Some(u) = state.active_vertices.pop_front() {
        if !state.push(u) {
          eprintln!("relabel {}", u);
          state.relabel(u);
        } else {
          eprintln!("push {}", u);
        }
        eprintln!("excess: {:?}", &state.excess);
        eprintln!("label: {:?}", &state.label);
      }

      state.excess[t]
    }

    /// BFSで最短パスを求める
    pub fn shortest_path_bfs<T: Measure>(&self, from: usize) -> Vec<Option<T>> {
      let mut dist = vec![None; self.vertices.len()];
      dist[from] = Some(T::zero());
      self.bfs(from, |e| {
        let edge = self.edge(e);
        dist[edge.to] = dist[edge.from].map(|d| d + T::one() );
      });
      dist
    }

    /// BFSする
    pub fn bfs<F: FnMut(usize)>(&self, from: usize, mut f: F) {
      self.walk(from, |walk, u| {
        for &e in &self.vertex(u).edges {
          if walk.go_next(self.edge(e).to) {
            (f)(e);
          }
        }
      });
    }

    /// DFSする
    pub fn dfs<F: FnMut(usize)>(&self, from: usize, mut f: F) {
      self.walk(from, |walk, u| {
        for &e in &self.vertex(u).edges {
          if walk.go_later(self.edge(e).to) {
            (f)(e);
          }
        }
      });
    }

    /// グラフ上の探索
    /// 01BFSなどに使える
    pub fn walk<F: FnMut(&mut Walk, usize)>(&self, from: usize, mut transport: F) {
      let mut walk = Walk { visited: vec![false; self.vertices.len()], queue: VecDeque::new() };
      walk.go_next(from);
      while let Some(u) = walk.next() {
        (transport)(&mut walk, u);
      }
    }

    // /// BFS木上のLCA
    // pub fn lca(&self, root: usize) -> Lca {
    //   let k = self.vertices.len().saturating_sub(1).next_power_of_two().trailing_zeros();
    //   let mut parent = vec![vec![None; self.vertices.len()]; k];
    //   let mut dist = vec![None; self.vertices.len()];
    //   dist[root] = Some(0);
    //   self.bfs(root, |e| {
    //     let edge = self.edge(e);
    //     parent[0][edge.to] = edge.from;
    //     dist[edge.to] = dist[edge.from].map(|d| d + 1 );
    //   });
    //   let mut ancestor = vec![vec![]; self.vertices.len()];
    //   Lca { ancestor, depth }
    // }
  }

  pub struct Walk {
    visited: Vec<bool>,
    queue: VecDeque<usize>,
  }
  impl Walk {
    // BFS
    pub fn go_next(&mut self, v: usize) -> bool {
      if self.visited[v] { return false; }
      self.visited[v] = true;
      self.queue.push_front(v);
      true
    }

    // DFS
    pub fn go_later(&mut self, v: usize) -> bool {
      if self.visited[v] { return false; }
      self.visited[v] = true;
      self.queue.push_back(v);
      true
    }

    pub fn unvisit(&mut self, v: usize) {
      self.visited[v] = false;
    }

    pub fn next(&mut self) -> Option<usize> {
      self.queue.pop_back()
    }
  }

  pub struct Lca {
  }
  impl Lca {
    pub fn query(u: usize, v: usize) -> usize {
      0
    }
  }

  // impls

  pub trait OptionUtil<T>: Sized {
    fn unwrap(self) -> T;
    fn is_some(&self) -> bool;
    fn insert(&mut self, value: T) -> &mut T;
    fn chmin(&mut self, other: T) -> &mut Self where Self: Clone, T: Clone + Ord {
      let value = if self.is_some() { self.clone().unwrap().min(other) } else { other };
      self.insert(value);
      self
    }
    fn chmax(&mut self, other: T) -> &mut Self where Self: Clone, T: Clone + Ord {
      let value = if self.is_some() { self.clone().unwrap().max(other) } else { other };
      self.insert(value);
      self
    }
    fn and_if<F: FnOnce(T) -> bool>(self, predicate: F) -> bool {
      self.is_some() && predicate(self.unwrap())
    }
    fn if_chmin<F: FnOnce()>(&mut self, other: T, procedure: F) -> &mut Self where Self: Clone, T: Clone + Ord {
      if !self.is_some() || self.clone().unwrap() > other {
        self.insert(other);
        procedure();
      }
      self
    }
    fn if_chmax<F: FnOnce()>(&mut self, other: T, procedure: F) -> &mut Self where Self: Clone, T: Clone + Ord {
      if !self.is_some() || self.clone().unwrap() < other {
        self.insert(other);
        procedure();
      }
      self
    }
  }
  impl<T> OptionUtil<T> for Option<T> {
    fn unwrap(self) -> T {
      Option::<T>::unwrap(self)
    }
    fn is_some(&self) -> bool {
      Option::<T>::is_some(self)
    }
    fn insert(&mut self, value: T) -> &mut T {
      *self = Some(value);
      self.as_mut().unwrap()
    }
  }

  impl<T: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign> AssignOps for T {}
  impl<T: std::fmt::Debug + Copy + Ord + Default + Num + AssignOps + std::iter::Sum> Measure for T {}
  impl<T: Signed + Measure> MeasureSigned for T {}

  impl<V> IntoVertices<V, std::vec::IntoIter<V>> for Vec<V> {
    fn into_vertices(self) -> std::vec::IntoIter<V> {
      self.into_iter()
    }
  }

  pub struct CloneIter<T: Clone> {
    count: usize,
    value: T,
    phantom: PhantomData<T>,
  }
  impl<T: Clone> CloneIter<T> {
    fn new(value: T, count: usize) -> Self {
      Self { value, count, phantom: PhantomData }
    }

    fn from_default(count: usize) -> Self where T: Default {
      Self::new(Default::default(), count)
    }
  }
  impl<T: Clone> Iterator for CloneIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
      if self.count == 0 {
        None
      } else {
        self.count -= 1;
        Some(self.value.clone())
      }
    }
  }

  impl<V: Clone + Default> IntoVertices<V, CloneIter<V>> for usize {
    fn into_vertices(self) -> CloneIter<V> {
      CloneIter::from_default(self)
    }
  }

  impl<E> IntoEdge<E> for (usize, usize, E) {
    fn into_edge(self) -> (usize, usize, E) {
      self
    }
  }
  impl<E: Default> IntoEdge<E> for (usize, usize) {
    fn into_edge(self) -> (usize, usize, E) {
      (self.0, self.1, Default::default())
    }
  }
}
