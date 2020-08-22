# Verified: https://atcoder.jp/contests/atc001/submissions/16154602

class UnionFind
	Node = Struct.new(:p, :h)
	def initialize(size); @p, @r = size.times.to_a, [0]*size; end
	def unite(i, j); k, l = parent(i), parent(j); if @r[k] < @r[l]; @p[k] = l; @r[l] += 1 if @r[k] == @r[l]; else; @p[l] = k; end; end
	def united?(i, j); parent(i) == parent(j); end
	def parent(i); j = i; until i == @p[i]; j, i = i, @p[j] = @p[i]; end; i; end
end
