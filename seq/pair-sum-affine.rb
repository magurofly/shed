# xs の非順序対の積と和の総和を計算する
# ```
# pair_sum_affine(xs, ab, a, b, c) == xs.combination(2).sum { ab * _1 * _2 + a * _1 + b * _2 + c }
# ```
def pair_sum_affine(xs, ab, a, b, c)
  n = xs.size
  ans = 0
  sum = 0
  (n - 1).downto(0) do |i|
    ans += (n - i - 1) * (a * xs[i] + c) + (ab * xs[i] + b) * sum
    sum += xs[i]
  end
  ans
end
