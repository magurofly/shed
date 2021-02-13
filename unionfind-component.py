class UnionFind:
  def __init__(self, size):
    self.parent = list(range(size))
    self.component = [[i] for i in range(size)]
  
  def root(self, i):
    if self.parent[i] != i:
      self.parent[i] = self.root(self.parent[i])
    return self.parent[i]
  
  def unite(self, i, j):
    i, j = self.root(i), self.root(j)
    if len(self.component[i]) < len(self.component[j]):
      i, j = j, i
    self.parent[j] = i
    self.component[i] += self.component[j]
