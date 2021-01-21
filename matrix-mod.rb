class ModMatrix
  attr_reader :n, :m, :rows
  def initialize(rows)
    @n, @m, @rows = rows.size, rows[0].size, rows
  end

  def self.id(n)
    new((0 ... n).map { |i|
      (0 ... n).map { |j| (i == j) ? 1 : 0 }
    })
  end

  def [](i, j)
    @rows[i][j]
  end

  def +(other)
    self.class.new(@rows.map.with_index { |row, i|
      row.map.with_index { |x, j| (x + other[i, j]) % MOD }
    })
  end

  def *(other)
    self.class.new((0 ... @n).map { |i|
      (0 ... other.n).map { |j|
        (0 ... @m).sum { |k| self[i, k] * other[k, j] } % MOD
      }
    })
  end

  def **(e)
    r = self.class.id(@n)
    x = self
    while e > 0
      r *= x if (e & 1) == 1
      x *= x
      e >>= 1
    end
    r
  end
end
