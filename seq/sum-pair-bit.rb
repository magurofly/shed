# xs の非順序対のANDの総和を計算する
# 非負整数のみ
# ```
# sum_pair_and(xs) == xs.combination(2).sum { _1 & _2 }
# ```
# 計算量: xs の長さを N 、ビット数を w とすると、 O(Nw)
def sum_pair_and(xs)
  n = xs.size
  m = xs.map(&:bit_length).max
  (0 ... m).sum do |bit|
    ans = 0
    sum = 0
    (n - 2).downto(0) do |i|
      sum += xs[i + 1][bit]
      ans += xs[i][bit] * sum
    end
    ans << bit
  end
end

# xs の非順序対のORの総和を計算する
# 非負整数のみ
# ```
# sum_pair_or(xs) == xs.combination(2).sum { _1 | _2 }
# ```
# 計算量: xs の長さを N 、ビット数を w とすると、 O(Nw)
def sum_pair_or(xs)
  n = xs.size
  m = xs.map(&:bit_length).max
  (0 ... m).sum do |bit|
    ans = 0
    sum = 0
    (n - 2).downto(0) do |i|
      sum += xs[i + 1][bit]
      ans +=
        case xs[i][bit]
        when 0
          sum
        when 1
          n - i - 1
        end
    end
    ans << bit
  end
end

# xs の非順序対のXORの総和を計算する
# 非負整数のみ
# ```
# sum_pair_xor(xs) == xs.combination(2).sum { _1 ^ _2 }
# ```
# 計算量: xs の長さを N 、ビット数を w とすると、 O(Nw)
def sum_pair_xor(xs)
  n = xs.size
  m = xs.map(&:bit_length).max
  (0 ... m).sum do |bit|
    ans = 0
    count = [0, 0]
    (n - 2).downto(0) do |i|
      count[xs[i + 1][bit]] += 1
      ans +=
        case xs[i][bit]
        when 0
          count[1]
        when 1
          count[0]
        end
    end
    ans << bit
  end
end
