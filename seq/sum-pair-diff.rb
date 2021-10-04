# xs の非順序対の差の総和を計算する
# ```
# pair_sum_diff(xs) == xs.combination(2).sum { _1 - _2 }
# ```
def pair_sum_diff(xs)
  n = xs.size
  ans = 0
  sum = 0
  (n - 2).downto(0) do |i|
    sum += xs[i + 1]
    ans += (n - i - 1) * xs[i] - sum
  end
  ans
end
