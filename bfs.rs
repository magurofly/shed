// つかいかた https://atcoder.jp/contests/abc184/submissions/20629427
// 注意：かなり遅い
struct BFS<P, C> {
  dist: HashMap<P, C>,
  queue: VecDeque<P>,
}
impl<P: Copy + cmp::Eq + hash::Hash, C: PrimInt> BFS<P, C> {
  fn new() -> Self { Self { dist: HashMap::new(), queue: VecDeque::new() } }
  fn bfs<F: FnMut(P, C, &mut BFS<P, C>) -> bool>(start: P, mut f: F) -> Option<C> {
    let mut this = Self::new();
    this.dist.insert(start, C::zero());
    this.queue.push_back(start);
    while let Some(pos) = this.queue.pop_front() {
      let cost = *this.dist.get(&pos).unwrap();
      if (f)(pos, cost, &mut this) { return Some(cost); }
    }
    return None;
  }
  fn next(&mut self, pos: P, cost: C) {
    if self.dist.contains_key(&pos) { return; }
    self.dist.insert(pos, cost);
    self.queue.push_back(pos);
  }
}
