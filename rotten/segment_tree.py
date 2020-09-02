class segment_tree:
	def __init__(self, a, id, op):
		m = 1
		while m < len(a): m <<= 1
		self.a, self.n, self.f = [id] * m + a, m, op
		i = m + len(a) - 1
		while i > 2:
			self.a[i>>1] = self.f(self.a[i-1], self.a[i])
			i -= 2
	def get(self, i): return self.a[i + self.n]
	def set(self, i, x):
		i += self.n
		self.a[i] = x
		while i > 1:
			i >>= 1
			self.a[i] = self.f(self.a[i<<1], self.a[i<<1|1])
	def fold(self, l, r):
		l += self.n
		r += self.n
		x = y = self.a[0]
		while l < r:
			if (l & 1) == 1:
				x = self.f(x, self.a[l])
				l += 1
			if (r & 1) == 1:
				r -= 1
				y = self.f(self.a[r], y)
			l >>= 1
			r >>= 1
		return self.f(x, y)
