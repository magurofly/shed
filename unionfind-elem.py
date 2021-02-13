class UnionFind:
  """
  uf = UnionFind([{1}, {2}, {1, 3}], lambda x, y: x | y)
  uf.unite(1, 2)
  print(uf.element)
  """
  def __init__(self, array, merge):
    self.parent = list(range(len(array)))
    self.size = [1] * len(array)
    self.element = array
    self.merge = merge
  
  def root(self, i):
    if self.parent[i] != i:
      self.parent[i] = self.root(self.parent[i])
    return self.parent[i]
  
  def unite(self, i, j):
    i, j = self.root(i), self.root(j)
    if self.size[i] < self.size[j]: i, j = j, i
    self.parent[j] = i
    self.element[i] = self.merge(self.element[i], self.element[j])
