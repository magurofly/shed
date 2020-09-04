struct BIT<T> { a: Vec<T>, n: usize }
impl<T: Copy + std::ops::AddAssign<T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + Default> BIT<T> {
	fn new(n: usize) -> BIT<T> { BIT { a: vec![Default::default(); n + 1], n } }
	fn update(&mut self, r: std::ops::Range<usize>, x: T) {
		if r.start != 0 { self.update(0..r.end, x); self.update(0..r.start-1, -x); return; }
		let mut i = r.end as i64;
		while i < self.n as i64 { self.a[i as usize] += x; i += i & -i; }
	}
	fn fold(&self, r: std::ops::Range<usize>) -> T {
		if r.start != 0 { return self.fold(0..r.end) - self.fold(0..r.start-1); }
		let (mut i, mut x) = (r.end as i64, self.a[0]);
		while i > 0 { x += self.a[i as usize]; i -= i & -i; }
		x
	}
}
