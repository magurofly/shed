# xs の非順序対の和の総和を計算する
# ```
# sum_pair_sum(xs) == xs.combination(2).sum { _1 + _2 }
# sum_pair_sum(xs, a, b, c) == xs.combination(2).sum { a * _1 + b * _2 + c }
# ```
def sum_pair_sum(xs, a = 1, b = 1, c = 0)
  n = xs.size
  ans = 0
  sum = 0
  (n - 1).downto(0) do |i|
    ans += (n - i - 1) * (a * xs[i] + c) + b * sum
    sum += xs[i]
  end
  ans
end
