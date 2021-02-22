use std::collections::BinaryHeap;
use num_traits::PrimInt;

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
      if dists[v] == None || dists[v].unwrap() > d2 {
        dists[v] = Some(d2);
        pq.push((cr - d, v));
      }
    }
  }
  dists
}
