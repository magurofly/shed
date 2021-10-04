# xs の非順序対の積の総和を計算する
# ```
# sum_pair_mul(xs) == xs.combination(2).sum { _1 * _2 }
# ```
def sum_pair_mul(xs)
  ans = 0
  sum = 0
  (xs.size - 2).downto(0) do |i|
    sum += xs[i + 1]
    ans += xs[i] * sum
  end
  ans
end
