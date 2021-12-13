def spfa(graph, start, inf = 10**18)
  n = graph.size
  dist = Array.new(n, inf)
  dist[start] = 0
  count = Array.new(n, 0)
  count[start] = 1
  in_queue = Array.new(n, false)
  in_queue[start] = true
  queue = [start]
  while (u = queue.shift)
    in_queue[u] = false
    d = dist[u]
    graph[u].each do |v, c|
      d2 = (d >= inf) ? inf : d + c
      next if dist[v] <= d2
      dist[v] = d2
      next if in_queue[v]
      in_queue[v] = true
      queue << v
      count[v] += 1
      dist[v] = inf if count[v] >= n
      break if count[v] >= n * 2
    end
  end
  dist
end
