// つかいかた https://atcoder.jp/contests/abc184/submissions/20630048
struct BFS<P, C> {
  dist: rustc_hash::FxHashMap<P, C>,
  queue: VecDeque<P>,
}
impl<P: Copy + cmp::Eq + hash::Hash, C: PrimInt> BFS<P, C> {
  fn new() -> Self { Self { dist: rustc_hash::FxHashMap::default(), queue: VecDeque::new() } }
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
