class BitAndOrMatrix
  attr_reader :n, :bits
  def initialize(n, bits)
    @n, @bits = n, bits
    @u = (1 << n) - 1
    @v = ((@u + 1)**n - 1) / @u
  end

  def self.rows(rows)
    n, bits = rows.size, 0
    rows.each_with_index do |row, i|
      x = 0
      row.each_with_index do |a, j|
        x |= a << j
      end
      bits |= x << n * i
    end
    new(n, bits)
  end

  def self.id(n)
    new(n, ((1 << (n + 1) * n) - 1) / ((1 << n + 1) - 1))
  end

  def [](i, j)
    @bits >> i * @n + j & 1
  end

  def +(other)
    self.class.new(@n, @bits | other.bits)
  end

  def *(other)
    a, b = @bits, other.bits
    bits = 0
    while a > 0 and b > 0
      bits |= ((a & @v) * @u) & ((b & @u) * @v)
      a >>= 1
      b >>= n
    end
    self.class.new(@n, bits)
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

  def to_a
    (0 ... @n).map { |i| (0 ... @n).map { |j| self[i, j] } }
  end
end
