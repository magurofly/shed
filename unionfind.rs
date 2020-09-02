struct UnionFind(Vec<usize>, Vec<usize>);
impl UnionFind {
	fn new(size: usize) -> Self { Self((0..size).collect(), vec![0; size]) }
	fn unite(&mut self, i: usize, j: usize) { let (mut k, mut l) = (self.find(i), self.find(j)); if self.1[k] < self.1[l] { std::mem::swap(&mut k, &mut l) }; if self.1[k] == self.1[l] { self.1[k] += 1 }; self.0[l] = self.0[k]; }
	fn find(&mut self, mut i: usize) -> usize { let mut j = i; while i != self.0[i] { self.0[j] = self.0[i]; j = i; i = self.0[i] }; i }
	fn is_same(&mut self, i: usize, j: usize) -> bool { self.find(i) == self.find(j) }
}
