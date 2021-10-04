# xs の非順序対の差の絶対値の総和を計算する
# ```
# pair_diff_abs_sum(xs) == xs.combination(2).sum { (_1 - _2).abs }
# ```
def pair_diff_abs_sum(xs)
  xs = xs.sort.reverse
  n = xs.size
  ans = 0
  sum = 0
  (n - 2).downto(0) do |i|
    sum += xs[i + 1]
    ans += (n - i - 1) * xs[i] - sum
  end
  ans
end
