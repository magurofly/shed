#![allow(non_snake_case)]

// #[fastout]
fn main() {
  let mut graph = MFGraph::<i64>::with_vertices(5);
  let edges = vec![(0, 1, 3), (0, 2, 3), (1, 2, 2), (1, 3, 1), (2, 3, 3), (2, 4, 2), (3, 4, 2)];
  println!("{:?}", &edges);
  graph.add_edges(edges);
  let flow = graph.maxflow_push_relabel(0, 4);
  println!("{}", flow);
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
  }
  pub trait MeasureSigned: Measure + Signed {}

  #[derive(Debug, Default)]
  pub struct Vertex<V> {
    id: usize,
    weight: V,
    edges: Vec<usize>,
  }

  #[derive(Debug, Default)]
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

  pub trait Graph<V, E> {
    fn vertex(&self, id: usize) -> &Vertex<V>;
    fn edge(&self, id: usize) -> &Edge<E>;
  }

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
    pub fn new() -> Self {
      Self {
        edge_type: PhantomData,
        vertices: Vec::new(),
        edges: Vec::new(),
      }
    }

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

    pub fn add_vertices<I: Iterator<Item = V>, Vs: IntoVertices<V, I>>(&mut self, vertices: Vs) -> Vec<usize> {
      vertices.into_vertices().map(|v| self.add_vertex(v) ).collect::<Vec<_>>()
    }

    pub fn connect(&mut self, from: usize, to: usize) -> usize where E: Clone + Default {
      self.add_edge(from, to, Default::default())
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: E) -> usize where E: Clone {
      D::add_edge(self, from, to, weight)
    }

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

    pub fn edges_from(&self, from: usize) -> Vec<usize> {
      self.vertex(from).edges.iter().copied().collect::<Vec<_>>()
    }

    pub fn adjacent_vertices(&self, from: usize) -> Vec<usize> {
      self.vertex(from).edges.iter().map(|&e| self.edge(e).to ).collect::<Vec<_>>()
    }

    pub fn dijkstra<T, F: FnMut(&Edge<E>) -> Option<T>>(&self, from: usize, mut cost_by: F) -> Vec<Option<T>> where T: Measure {
      assert!(from < self.vertices.len());
      let mut dist = vec![None; self.vertices.len()];
      dist[from] = Some(T::zero());
      let mut pq = BinaryHeap::new();
      pq.push((Reverse(T::zero()), from));
      while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] != Some(d) { continue; }
        for edge in self.vertices[u].edges.iter().map(|&e| self.edge(e) ) {
          if let Some(cost) = (cost_by)(edge) {
            if dist[edge.to].map(|x| x <= d + cost ).unwrap_or(false) { continue; }
            dist[edge.to] = Some(d + cost);
            pq.push((Reverse(d + cost), edge.to));
          }
        }
      }
      dist
    }

    pub fn floyd_warshall<T, F: FnMut(&Edge<E>) -> Option<T>>(&self, make_loop: bool, mut cost_by: F) -> Vec<Vec<Option<T>>> where T: Measure {
      let vertices = self.vertices.len();
      let mut dist: Vec<Vec<Option<T>>> = vec![vec![None; vertices]; vertices];
      for edge in &self.edges {
        if let Some((d, cost)) = zip(dist[edge.from][edge.to].as_mut(), (cost_by)(edge)) {
          d.chmin(cost);
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
              if let Some(d) = dist[i][j].as_mut() {
                d.chmin(d1 + d2);
              } else {
                dist[i][j] = Some(d1 + d2);
              }
            }
          }
        }
      }
      dist
    }

    // pub fn spfa<T, F: FnMut(&Edge<E>) -> Option<T>>(&self, from: usize, mut cost_by: F) -> Vec<Result<T, Status>>

    // Generic Push/Relabel + Global Relabeling + FIFO
    // @param s source
    // @param t sink
    // @return flow
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
          for (v, d) in self.graph.bfs(self.sink).into_iter().enumerate() {
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

    pub fn bfs<T: Measure>(&self, from: usize) -> Vec<Option<T>> {
      self.bfs_by(from, |bfs, u| {
        for v in self.adjacent_vertices(u) {
          bfs.go_next(v);
        }
      })
    }

    pub fn bfs_by<T: Measure, F: FnMut(&mut Bfs<T>, usize)>(&self, from: usize, mut transport: F) -> Vec<Option<T>> {
      let mut bfs = Bfs { current: from, dist: vec![None; self.vertices.len()], queue: VecDeque::new() };
      bfs.dist[from] = Some(T::zero());
      bfs.queue.push_back(from);
      while let Some(u) = bfs.queue.pop_back() {
        bfs.current = u;
        (transport)(&mut bfs, u);
      }
      bfs.dist
    }
  }

  pub struct Bfs<T> {
    current: usize,
    dist: Vec<Option<T>>,
    queue: VecDeque<usize>,
  }
  impl<T: Measure> Bfs<T> {
    // BFS
    pub fn go_next(&mut self, v: usize) {
      if self.dist[v].is_some() { return; }
      self.dist[v] = self.dist[self.current].map(|x| x + T::one() );
      self.queue.push_front(v);
    }

    // DFS
    pub fn go_last(&mut self, v: usize) {
      if self.dist[v].is_some() { return; }
      self.dist[v] = self.dist[self.current].map(|x| x + T::one() );
      self.queue.push_back(v);
    }
  }

  // impls

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
