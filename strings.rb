# 文字列に対して prefix[i] = max { k | s[0, k] == s[i, k] } を返す
def z_algorithm(s)
    prefix = [0] * s.size
    j = 0
    (1 ... s.size).each do |i|
      if i + prefix[i - j] < j + prefix[j]
        prefix[i] = prefix[i - j]
      else
        k = [0, j + prefix[j] - i].max
        k += 1 while i + k < s.size and s[k] == s[i + k]
        prefix[i] = k
        j = i
      end
    end
    prefix[0] = s.size
    prefix
end

# 回文
class Parindromes
  def self.manacher(str)
    n = str.size
    i = j = 0
    r = Array.new(n, 0)
    while i < n
      j += 1 while i >= j and i + j < n and str[i - j] == str[i + j]
      r[i] = j
      k = 1
      while i >= k and k + r[i - k] < j
        r[i + k] = r[i - k]
        k += 1
      end
      i += k
      j -= k
    end
    r
  end

  def initialize(str, sep = ?$)
    @r = Parindromes.manacher(str.chars.join(sep))
  end

  def count_all
    @r.each_with_index.sum { |r, i| (r + 1 - (i & 1)) >> 1 }
  end

  # 文字を中心とする回文（奇数長）の半径
  def odd(i)
    (@r[i * 2] + 1) >> 1
  end

  # 文字の間を中心とする回文（偶数長）の半径
  def even(i)
    (@r[i * 2 + 1] - 1) >> 1
  end
end
