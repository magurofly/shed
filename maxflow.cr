class MaxFlow(T)
  class Arc(T)
    property from, to, cap, flow
    def initialize(@from : Int32, @to : Int32, @cap : T, @flow); end
    def residual_cap; cap - flow end
  end

  @graph = [] of Array(Int32)
  @arcs = [] of Arc(T)
  @cap_sum : T = 0
  # ネットワークを初期化
  def initialize(n : Int32 = 0); add_vertices(n) end
  # 頂点数
  def n; @graph.size end
  def m; @arcs.size // 2 end
  def arc(e : Int32); @arcs[e * 2] end
  def add_vertex; v = n; @graph << [] of Int32; v end
  def add_vertices(n : Int32); (0 ... n).map { add_vertex } end
  def add_arc(from u : Int32, to v : Int32, cap c : Int32); e = m; @arcs << Arc(T).new(u, v, c, 0) << Arc(T).new(v, u, c, c); @graph[u] << (e * 2); @graph[v] << (e * 2 + 1); @cap_sum += c; e end

  # source から sink へ最大 limit 流す
  def flow(source s : Int32, sink t : Int32, limit : T = @cap_sum)
    return 0 if limit == 0
    flow_sum = 0
    (0 ... limit.bit_length).reverse_each do |bit|
      flow = 1 << bit
      loop do
        level = bfs(s)
        break if level[t] == n
        flow += dfs(level, [0] * n, s, t, flow, limit - flow_sum)
      end
    end
  end

  # 残余ネットワーク上で source からの到達判定
  def cut(source : Int32)
    visited = [false] * n
    visited[source] = true
    stack = [source]
    while (u = stack.pop?)
      @graph[u].each do |e|
        arc = @arcs[e]
        next if visited[arc.to] || arc.residual_cap <= 0
        visited[arc.to] = true
        stack << arc.to
      end
    end
    visited
  end

  private def bfs(s)
    level = [n] * n
    level[s] = 0
    queue = Deque.new([s])
    while (u = queue.shift?)
      @graph[u].each do |e|
        arc = @arcs[e]
        next if level[arc.to] < level[u] + 1 || arc.residual_cap <= 0
        level[arc.to] = level[u] + 1
        queue << arc.to
      end
    end
    level
  end

  private def dfs(level, iter, u, t, base, limit)
    return flow if u == t
    flow_sum = 0
    while iter[u] < @graph[u].size
      e = @graph[u][iter[u]]
      arc = @arcs[e]
      if level[u] < level[arc.to] && arc.residual_cap >= base
        delta = dfs(level, iter, arc.to, t, base, [arc.residual_cap, limit - flow_sum].min)
        if delta > 0
          arc.flow += delta
          @arcs[e ^ 1].flow -= delta
          flow_sum += delta
          break if limit - flow_sum < base
        end
      end
      iter[u] += 1
    end
    flow_sum
  end
end
