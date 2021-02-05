// ARC109 A - Hands
use petgraph::{graph::UnGraph, algo::dijkstra};
use proconio::{input, fastout, marker::Usize1};

#[fastout]
fn main() {
  input!(a: Usize1, b: Usize1, x: i32, y: i32);
  // 0-99: A, 100-199: B
  
  // edges is vector<(from, to)> or vector<(from, to, weight)>
  let mut edges = vec![];
  for i in 0 .. 100 {
    edges.push((i, i+100, x));
  }
  for i in 0 .. 99 {
    edges.push((i+1, i+100, x));
    edges.push((i, i+1, y));
    edges.push((i+100, i+101, y));
  }
  
  // initializing undirected graph (petgraph::graph::UnGraph<N, E>); directed graph is petgraph::graph::DiGraph
  let mut graph = UnGraph::<(), i32>::from_edges(edges);
  
  // node_indices to specify vertex
  let mut nodes = graph.node_indices().collect::<Vec<_>>();
  
  // compute shortest path costs; dijkstra returns HashMap<NodeIndex, Cost>
  let cost = *dijkstra(&graph, nodes[a], None, |e| *e.weight()).get(&nodes[b+100]).unwrap();
  
  println!("{}", cost);
}
