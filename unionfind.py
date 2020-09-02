class unionfind:
	def __init__(self, size): self.parent, self.rank = list(range(size)), [0] * size
	def unite(self, i, j):
		k, l = self.find(i), self.find(j)
		if self.rank[k] > self.rank[l]: k, l = l, k
		if self.rank[k] == self.rank[l]: self.rank[k] += 1
		self.parent[l] = k
	def find(self, i):
		j = i
		while i != self.parent[i]:
			self.parent[j] = self.parent[i]
			j, i = i, self.parent[j]
		return i
	def same(self, i, j): return self.find(i) == self.find(j)
