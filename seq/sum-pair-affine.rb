# xs の非順序対の積と和の総和を O(N) で計算する
# ```
# sum_pair_affine(xs, aa, bb, ab, a, b, c) == xs.combination(2).sum { aa * _1**2 + bb * _2**2 * ab * _1 * _2 + a * _1 + b * _2 + c }
# ```
def sum_pair_affine(xs, aa = 0, bb = 0, ab = 0, a = 0, b = 0, c = 0)
  n = xs.size
  ans = 0
  sum = 0
  (n - 1).downto(0) do |i|
    ans += (aa * (n - i - 1) + bb * i) * xs[i]**2 + (n - i - 1) * (a * xs[i] + c) + (ab * xs[i] + b) * sum
    sum += xs[i]
  end
  ans
end

def sum_pair_affine_mod(xs, mod, aa = 0, bb = 0, ab = 0, a = 0, b = 0, c = 0)
  n = xs.size
  ans = 0
  sum = 0
  (n - 1).downto(0) do |i|
    ans += (aa * (n - i - 1) + bb * i) % mod * xs[i]**2 % mod + (n - i - 1) * (a * xs[i] % mod + c) % mod + (ab * xs[i] + b) * sum % mod
    sum += xs[i]
    ans %= mod
    sum %= mod
  end
  ans % mod
end

# ABC194 C - Squared Error
# ```
# N = gets.to_i
# A = gets.split.map(&:to_i)
# puts sum_pair_affine(A, 1, 1, -2)  # (x - y)**2 == x**2 + y**2 - 2*x*y
# ```
# https://atcoder.jp/contests/abc194/submissions/26362955


# ABC177 C - Sum of product of pairs
# ```
# MOD = 10**9 + 7
# N = gets.to_i
# A = gets.split.map(&:to_i)
# puts sum_pair_affine_mod(A, MOD, 0, 0, 1)
# ```
# https://atcoder.jp/contests/abc177/submissions/26362983
