# 無向木オンライン削除クエリ

- `cut(e)`: `e` 番目の辺を削除する O(logN)
- `size(v)`: `v` 番目の頂点が属する連結成分の大きさ O(1)

```ruby
class ReverseUnionFind
  attr_reader :components
  
  def initialize(n, edges)
    require "set"
    @components = 1
    @component = Array.new(n, Set.new(0 ... n))
    @graph = Array.new(n) { [] }
    @edges = edges.map { |u, v|
      e = [v, false]
      @graph[u] << e
      f = [u, false]
      @graph[v] << f
      [e, f]
    }
    @visited = Array.new(n, false)
  end
  
  # i 番目の辺を削除する（すでに削除済なら何もしない）
  def cut(i)
    e, f = @edges[i]
    return false if e[1] or f[1]
    e[1] = f[1] = true
    x, y = get_min(e[0], f[0])
    x = Set.new(x)
    y = @component[y[0]]
    x.each do |v|
      @component[v] = x
      y.delete(v)
    end
    @components += 1
    true
  end
  
  # v が属する連結成分
  def [](v)
    @component[v]
  end
  
  # v が属する連結成分の大きさ
  def size(v)
    @component[v].size
  end
  
  private
  
  # s, t それぞれの連結成分のうち、小さい方と大きい方の頂点リストを返す
  # ただし大きい方は切り詰めている
  def get_min(s, t)
    f = ->(a) {
      stack = [a]
      @visited[a] = true
      component = [a]
      Fiber.yield component
      while (u = stack.pop)
        Fiber.yield false
        @graph[u].each do |v, is_cut|
          next if is_cut or @visited[v]
          @visited[v] = true
          component << v
          stack << v
        end
      end
      Fiber.yield true
    }
    f1 = Fiber.new(&f)
    f2 = Fiber.new(&f)
    x1 = f1.resume(s)
    x2 = f2.resume(t)
    loop do
      r1 = f1.resume
      r2 = f2.resume
      if r1 or r2
        x1.each do |v|
          @visited[v] = false
        end
        x2.each do |v|
          @visited[v] = false
        end
        return r1 ? [x1, x2] : [x2, x1]
      end
    end
  end
end
```
