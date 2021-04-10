fn main() {
  let mut graph = graphs::DiGraph::<(), ()>::with_vertices(3);
  graph.add_edges(vec![(0, 1), (1, 2), (0, 2)]);
  println!("{:?}", graph.bfs(0));
  println!("{:?}", graph);
}


pub mod graphs {
  use std::marker::PhantomData;
  use std::collections::*;
  use std::cmp::*;
  use num_traits::*;
  
  pub trait AssignOps: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign {}
  pub trait Measure: Num + Default + Ord + Copy + AssignOps {
    fn min(self, other: Self) -> Self {
      if self > other {
        return other;
      }
      self
    }
    fn max(self, other: Self) -> Self {
      if self < other {
        return other;
      }
      self
    }
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
  }

  pub trait EdgeType {
    fn is_directed() -> bool;
  }
  #[derive(Debug)]
  pub enum Directed {}
  #[derive(Debug)]
  pub enum Undirected {}

  pub type DiGraph<V = (), E = ()> = VecGraph<V, E, Directed>;
  pub type UnGraph<V = (), E = ()> = VecGraph<V, E, Undirected>;

  pub trait IntoVertices<V, I: Iterator<Item = V>> {
    fn into_vertices(self) -> I;
  }
  pub trait IntoEdge<E> {
    fn into_edge(self) -> (usize, usize, E);
  }

  #[derive(Debug, Default)]
  pub struct VecGraph<V = (), E = (), D: EdgeType = Directed> {
    directed: PhantomData<D>,
    vertices: Vec<Vertex<V>>,
    edges: Vec<Edge<E>>,
  }
  impl<V, E, D: EdgeType> VecGraph<V, E, D> {
    pub fn new() -> Self {
      Self {
        directed: PhantomData,
        vertices: Vec::new(),
        edges: Vec::new(),
      }
    }

    pub fn with_vertices<I: Iterator<Item = V>, Vs: IntoVertices<V, I>>(vertices: Vs) -> Self {
      Self {
        directed: PhantomData,
        vertices: vertices.into_vertices().enumerate().map(|(id, weight)| Vertex { id, weight, edges: Vec::new() } ).collect::<Vec<_>>(),
        edges: Vec::new(),
      }
    }

    pub fn is_directed(&self) -> bool {
      D::is_directed()
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

    pub fn add_edge_directed(&mut self, from: usize, to: usize, weight: E) -> usize {
      assert!(from < self.vertices.len() && to < self.vertices.len());
      let id = self.edges.len();
      self.edges.push(Edge { id, weight, from, to });
      self.vertices[from].edges.push(id);
      id
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: E) -> usize where E: Clone {
      let id = self.add_edge_directed(from, to, weight.clone());
      if !D::is_directed() {
        self.add_edge_directed(to, from, weight);
      }
      id
    }

    pub fn add_edges<I: IntoIterator>(&mut self, edges: I) where E:Clone, I::Item: IntoEdge<E> {
      for edge in edges {
        let (from, to, weight) = edge.into_edge();
        self.add_edge(from, to, weight);
      }
    }

    pub fn vertex(&self, id: usize) -> &Vertex<V> {
      assert!(id < self.vertices.len());
      &self.vertices[id]
    }

    pub fn vertex_mut(&mut self, id: usize) -> &mut Vertex<V> {
      assert!(id < self.vertices.len());
      &mut self.vertices[id]
    }
    
    pub fn edge(&self, id: usize) -> &Edge<E> {
      assert!(id < self.edges.len());
      &self.edges[id]
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
        if let Some((d, cost)) = dist[edge.from][edge.to].as_mut().zip((cost_by)(edge)) {
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
            if let Some((d1, d2)) = dist[i][k].zip(dist[k][j]) {
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

    // pub fn spfa<T, F: FnMut(&Edge<E>) -> Option<T>>(&self)

    pub fn bfs(&self, from: usize) -> Vec<Option<usize>> {
      self.bfs_by(from, |bfs, u| {
        for v in self.adjacent_vertices(u) {
          bfs.go_next(v);
        }
      })
    }

    pub fn bfs_by<F: FnMut(&mut Bfs, usize)>(&self, from: usize, mut transport: F) -> Vec<Option<usize>> {
      let mut bfs = Bfs { current: from, dist: vec![None; self.vertices.len()], queue: VecDeque::new() };
      bfs.dist[from] = Some(0);
      bfs.queue.push_back(0);
      while let Some(u) = bfs.queue.pop_back() {
        bfs.current = u;
        (transport)(&mut bfs, u);
      }
      bfs.dist
    }
  }

  pub struct Bfs {
    current: usize,
    dist: Vec<Option<usize>>,
    queue: VecDeque<usize>,
  }
  impl Bfs {
    // BFS
    pub fn go_next(&mut self, v: usize) {
      if self.dist[v].is_some() { return; }
      self.dist[v] = self.dist[self.current].map(|x| x + 1 );
      self.queue.push_front(v);
    }

    // DFS
    pub fn go_last(&mut self, v: usize) {
      if self.dist[v].is_some() { return; }
      self.dist[v] = self.dist[self.current].map(|x| x + 1 );
      self.queue.push_back(v);
    }
  }

  // impls

  impl<T: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign> AssignOps for T {}
  impl<T: Copy + Ord + Default + Num + AssignOps> Measure for T {}
  impl<T: Signed + Measure> MeasureSigned for T {}

  impl EdgeType for Directed {
    fn is_directed() -> bool { true }
  }
  impl EdgeType for Undirected {
    fn is_directed() -> bool { false }
  }

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
