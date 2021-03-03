def graph_dfs(graph, start, visited = [false] * graph.size)
  visited[start] = true
  stack = [start]
  until stack.empty?
    u = stack.pop
    graph[u].each do |v|
      next if visited[v]
      visited[v] = true
      yield v, u
      stack << v
    end
  end
end

# @require graph_dfs
def graph_make_dfs_tree(graph, root = 0)
  tree = Array.new(graph.size) { [] }
  graph_dfs(graph, root) do |v|
    tree[u] << v
  end
  tree
end

# @require graph_dfs
def graph_connected_components(graph)
  components = []
  visited = [false] * graph.size
  graph.size.times do |start|
    next if visited[start]
    component = [start]
    graph_dfs(graph, start, visited) do |v|
      component << v
    end
    components << component
  end
  components
end
