#!title 単純なBFS
#!description 幅優先探索でグラフを探索する。第二引数に使用するキューを指定できる
#!tags グラフ 最短経路 BFS 幅優先探索 探索
require "set"
# @use Graph
def search_bfs(graph, start = 0, queue = [], visited = Set.new, &cond)
	visited << start; queue << start
	until queue.empty?
		i = queue.shift
		return i if cond[i]
		graph.connected(i) { |j| (visited << j; queue << j) unless visited.include? j }
	end
	nil
end
