fn main() {
  
}


pub mod graphs {
  use std::marker::PhantomData;
  use std::collections::*;
  use std::cmp::*;
  use num_traits::*;
  
  pub trait AssignOps: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign {}
  pub trait Measure: Num + Default + Ord + Copy + AssignOps {}
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
  pub enum Directed {}
  pub enum Undirected {}
  impl EdgeType for Directed {
    fn is_directed() -> bool { true }
  }
  impl EdgeType for Undirected {
    fn is_directed() -> bool { false }
  }


  type DiGraph<V, E> = VecGraph<V, E, Directed>;
  type UnGraph<V, E> = VecGraph<V, E, Undirected>;

  #[derive(Debug, Default)]
  pub struct VecGraph<V, E, D: EdgeType = Directed> {
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

    pub fn with_vertices(n: usize) -> Self where V: Default {
      Self {
        directed: PhantomData,
        vertices: (0 .. n).map(|id| Vertex { id, ..Default::default() } ).collect::<Vec<_>>(),
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

    pub fn add_edge(&mut self, from: usize, to: usize, weight: E) -> usize {
      assert!(from < self.vertices.len() && to < self.vertices.len());
      let id = self.edges.len();
      self.edges.push(Edge { id, weight, from, to });
      self.vertices[from].edges.push(id);
      if !D::is_directed() {
        self.vertices[to].edges.push(id);
      }
      id
    }

    pub fn vertex(&self, id: usize) -> &Vertex<V> {
      assert!(id < self.vertices.len());
      &self.vertices[id]
    }

    pub fn vertex_mut(&mut self, id: usize) -> &mut Vertex<V> {
      assert!(id < self.vertices.len());
      &mut self.vertices[id]
    }

    pub fn dijkstra<F: FnMut(usize, usize, E) -> Option<E>>(&self, from: usize, mut weight_by: F) -> Vec<Option<E>> where E: Measure {
      assert!(from < self.vertices.len());
      let mut dist = vec![None; self.vertices.len()];
      dist[from] = Some(E::zero());
      let mut pq = BinaryHeap::new();
      pq.push((Reverse(E::zero()), from));
      while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] != Some(d) { continue; }
        for e in &self.vertices[u].edges {
          let (from, to, weight) = {
            let edge = &self.edges[e >> 1];
            let mut from = edge.from;
            let mut to = edge.to;
            if u == to { std::mem::swap(&mut from, &mut to); }
            (from, to, edge.weight)
          };
          if let Some(weight) = (weight_by)(from, to, weight) {
            if dist[to].map(|x| x <= d + weight ).unwrap_or(false) { continue; }
            dist[to] = Some(d + weight);
            pq.push((Reverse(d + weight), to));
          }
        }
      }
      dist
    }

    pub fn bfs<F: FnMut(&mut Bfs, usize)>(&self, from: usize, mut transport: F) -> Vec<Option<usize>> {
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

  impl<T: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign> AssignOps for T {}
  impl<T: Copy + Ord + Default + Num + AssignOps> Measure for T {}
  impl<T: Signed + Measure> MeasureSigned for T {}
}
