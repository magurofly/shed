#!title グラフ（隣接行列）
#!description 隣接行列によるグラフ。追加O(1)、探索O(N)
#!tags データ構造 グラフ 隣接行列
class GraphByMatrix
	# @implements Graph
	attr_reader :size
	
	def initialize(n)
		@size = n
		@mat = n.times.map {[nil] * n}
	end

	def connect(i, j, cost = 1)
		@mat[i][j] = @mat[j][i] = cost
	end

	def connected(i)
		@mat[i].each_with_index { |c, j| yield j, c if c }
	end
end
