// 未Verify
pub fn shortest_path_dial(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<Option<usize>> {
  let cost_limit = graph.iter().filter_map(|edges| edges.iter().map(|edge| edge.1).max()).max().unwrap() + 1;
  let mut dist = vec![None; graph.len()];
  dist[start] = Some(0);
  let mut stacks = vec![vec![]; cost_limit];
  stacks[0].push((start, 0));
  let mut i = 0;
  let mut count = 1;
  while count != 0 {
    while let Some((u, d)) = stacks[i].pop() {
      count -= 1;
      if dist[u].unwrap() != d {
        continue;
      }
      for &e in &graph[u] {
        if dist[e.0].map(|d2| d2 <= d + e.1).unwrap_or(false) {
          continue;
        }
        dist[e.0] = Some(d + e.1);
        stacks[(i + e.1) % cost_limit].push(e);
        count += 1;
      }
    }
    i = (i + 1) % cost_limit;
  }
  dist
}
