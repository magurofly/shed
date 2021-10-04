# xs の非順序対のminの総和を計算する
# ```
# sum_pair_min(xs) == xs.combination(2).sum { [_1, _2].min }
# ```
def sum_pair_min(xs)
  xs = xs.sort
  n = xs.size
  (0 ... n - 1).sum { |i| (n - i - 1) * xs[i] }
end

# xs の非順序対のmaxの総和を計算する
# ```
# sum_pair_max(xs) == xs.combination(2).sum { [_1, _2].max }
# ```
def sum_pair_max(xs)
  xs = xs.sort.reverse
  n = xs.size
  (0 ... n - 1).sum { |i| (n - i - 1) * xs[i] }
end
