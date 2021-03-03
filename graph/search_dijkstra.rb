#!title ダイクストラ法
#!description 幅優先探索でグラフを探索する。第二引数に使用するキューを指定できる
#!tags グラフ 最短経路 ダイクストラ法 探索

# q_classにフィボナッチヒープなどを指定すると速くなる

# @return `start`からの最短経路の長さ
def search_dijkstra(graph, start = 0, q_class = Array, inf = Float::INFINITY)
	d = [inf] * graph.size
	d[start] = 0
	prev = [nil] * graph.size
	q = q_class.new { |i, j| d[i] <=> d[j] }
	graph.size.times { |i| q << i }
	until q.empty?
		i = q.min_by { |j| d[j] }
		q.delete i
		graph.connected(i) { |j, c| (d[j] = d[i] + c; prev[j] = i) }
	end
	d
end
