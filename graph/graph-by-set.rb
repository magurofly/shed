#!title グラフ（隣接集合）
#!description 隣接集合によるグラフ。追加O(N)、探索O(1)
#!tags データ構造 グラフ 隣接リスト
class GraphBySet
	# @implements Graph
	Node = Struct.new(:cost, :index)
	attr_reader :size

	def initialize(n, set_class = Array)
		@size = n
		@mat = n.times.map {set_class.new}
	end

	def connect(i, j, cost = 1)
		@mat[i].connected << Node[j, cost] unless @mat[i].include? j
	end

	def connected(i)
		@mat[i].each { |n| yield n.index, n.cost }
	end
end
