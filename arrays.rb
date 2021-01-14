# ソートされた配列の重複削除
def sorted_uniq(xs)
  prev = nil
  xs.filter { |x| c = prev != x; prev = x; c }
end

# ソートされた配列から要素を二分探索
def sorted_index(xs, y)
  xs.bsearch_index { |x| y <=> x }
end

# ソートされた配列の部分和を列挙
# @require sorted_merge
def enumerate_sum(arr)
  sums = [0]
  arr.each do |x|
    sums = sorted_merge(sums, sums.map { |y| x + y })
  end
  sums
end

# ソートされた配列を合成
def sorted_merge(xs, ys)
  zs = []
  i = j = 0
  n, m = xs.size, ys.size
  while i < n and j < m
    a, b = xs[i], ys[j]
    if a <= b
      zs << a
      i += 1
    else
      zs << b
      j += 1
    end
  end
  zs += ys[j..] if i == n
  zs += xs[i..] if j == n
  zs
end
