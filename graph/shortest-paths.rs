use std::collections::*;
use num_traits::PrimInt;

fn main() {
    let mut graph = vec![vec![(0, -1)]];
    let dists = spfa(&graph, 0);
    println!("{:?}", dists);
}

// Dijkstra 法により、辺に非負重みがあるグラフの単一始点最短経路を求める
// @complexity O((E+V)logE)
// @param graph 隣接リスト形式のグラフ
// @param start 始点の頂点番号
// @return 始点からそれぞれの頂点への最短経路（存在しない場合はNone）
fn dijkstra<C: PrimInt>(graph: &Vec<Vec<(usize, C)>>, start: usize) -> Vec<Option<C>> {
  let mut dists = vec![None; graph.len()];
  dists[start] = Some(C::zero());
  let mut pq = BinaryHeap::new(); // (-dist, vertex)
  pq.push((C::zero(), start));
  while let Some((cr, u)) = pq.pop() {
    let c = C::zero() - cr;
    if dists[u].is_some() && dists[u].unwrap() != c { continue; }
    for &(v, d) in graph[u].iter() {
      let d2 = c + d;
      if dists[v] == None || dists[v].unwrap() > d2 { continue; }
      dists[v] = Some(d2);
      pq.push((cr - d, v));
    }
  }
  dists
}

// Shortest Path Faster Algorithm (Bellman-Ford の定数倍高速化) により、グラフの単一始点最短経路を求める
// @complexity O(VE)
// @param graph 隣接リスト形式のグラフ
// @param start 始点の頂点番号
// @return 負の閉路が存在する場合はNone、そうでなければSome(始点からそれぞれの頂点への最短経路)
fn spfa<C: PrimInt>(graph: &Vec<Vec<(usize, C)>>, start: usize) -> Option<Vec<Option<C>>> {
  let mut dists = vec![None; graph.len()];
  let mut count = vec![0; graph.len()];
  let mut in_queue = vec![false; graph.len()];
  let mut queue = VecDeque::new();
  dists[start] = Some(C::zero());
  count[start] = 1;
  in_queue[start] = true;
  queue.push_back(start);
  while let Some(u) = queue.pop_front() {
    in_queue[u] = false;
    let c = dists[u].unwrap();
    for &(v, d) in graph[u].iter() {
      let d2 = c + d;
      if dists[v] != None && dists[v].unwrap() <= d2 { continue; }
      dists[v] = Some(d2);
      if in_queue[v] { continue; }
      count[v] += 1;
      if count[v] >= graph.len() { return None; }
      queue.push_back(v);
      in_queue[v] = true;
    }
  }
  Some(dists)
}
