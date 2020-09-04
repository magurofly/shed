struct SegTree<T> { a: Vec<T>, n: usize, f: fn(T, T) -> T }
impl<T: Copy> SegTree<T> {
	fn new(size: usize, id: T, f: fn(T, T) -> T) -> SegTree<T> {
		let mut n = 1;
		while n < size { n <<= 1; }
		let mut a = vec![id; n + size];
		SegTree { a, n, f }
	}
	fn get(&self, i: usize) -> T { self.a[i + self.n] }
	fn set(&mut self, mut i: usize, x: T) {
		i += self.n;
		self.a[i] = x;
		while i > 1 {
			i >>= 1;
			self.a[i] = (self.f)(self.a[i << 1 | 0], self.a[i << 1 | 1]);
		}
	}
	fn fold(&self, range: std::ops::Range<usize>) -> T {
		let (mut l, mut r) = (range.start + self.n, range.end + self.n);
		let (mut x, mut y) = (self.a[0], self.a[0]);
		while l < r {
			if (l & 1) == 1 {
				x = (self.f)(x, self.a[l]);
				l += 1;
			}
			l >>= 1;
			if (r & 1) == 1 {
				r -= 1;
				y = (self.f)(self.a[r], y);
			}
			r >>= 1;
		}
		(self.f)(x, y)
	}
}
