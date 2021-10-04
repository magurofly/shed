# xs の連続部分列の総和の総和を計算する
# ```
# sum_css_sum(xs) == [* 0 .. xs.size].combination(2).sum { |l, r| (l ... r).sum { |i| xs[i] } }
# ```
def sum_css_sum(xs)
  n = xs.size

  # 累積和
  ys = [0] + xs
  n.times do |i|
    ys[i + 1] += ys[i]
  end

  sum = 0
  ans = 0
  n.downto(0) do |i|
    ans += sum - (n - i) * ys[i]
    sum += ys[i]
  end

  ans
end
