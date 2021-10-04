# xs の連続部分列のminの総和を計算する
# 条件: xs は unique
# 計算量: O(N)
# ```
# xs.uniq!
# sum_css_min_1(xs) == [* 0 .. xs.size].combination(2).sum { |l, r| xs[l ... r].min }
# ```
# Ref: https://nebocco.hatenablog.com/entry/2020/08/22/112452
def sum_css_min_1(xs)
  n = xs.size
  buf = []
  left = [0] * (n + 1)
  right = [0] * (n + 1)

  # 左から
  0.upto(n - 1) do |i|
    buf.pop while not buf.empty? and xs[buf[-1]] > xs[i]
    left[i] = buf[-1] || -1
    buf << i
  end

  # 右から
  buf.clear
  right = []
  (n - 1).downto(0) do |i|
    buf.pop while not buf.empty? and xs[buf[-1]] > xs[i]
    right[i] = buf[-1] || n
    buf << i
  end

  (0 ... n).sum { |i| xs[i] * (i - left[i]) * (right[i] - i) }
end

# xs の連続部分列のminの総和を計算する
# 計算量: O(N log N)
# 依存ライブラリ: UnionFind
# ```
# sum_css_min_2(xs) == [* 0 .. xs.size].combination(2).sum { |l, r| xs[l ... r].min }
# ```
def sum_css_min_2(xs)
  ys = xs.each_cons(2).map.with_index { |(a, b), i| [[a, b].min, i] }
  ys.sort!
  ys.reverse!

  uf = UnionFind.new(xs.size)
  ans = xs.sum
  ys.each do |y, i|
    next if uf.same?(i, i + 1)
    ans += uf.size(i) * uf.size(i + 1) * y
    uf.merge(i, i + 1)
  end

  ans
end

class UnionFind
  def initialize(size); @p = Array.new(size, -1); end
  def leader(i); j = i; j, i = i, @p[j] = @p[i] until @p[i] < 0; i; end
  def merge(i, j); k, l = leader(i), leader(j); return false if k == l; k, l = l, k if @p[k] > @p[l]; @p[k] += @p[l]; @p[l] = k; true; end
  def same?(i, j); leader(i) == leader(j); end
  def size(i); -@p[leader(i)]; end
end
