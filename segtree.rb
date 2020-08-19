class SegTree
  Info = Struct.new(:id, :op)

  def initialize(size, id, fill = id, &op)
    unless block_given?
      raise "#{id} is not #{Info}" unless id.is_a? Info
      id, op = id.id, id.op
    end
    n = 1
    n <<= 1 while n < size
    @op, @n, @a = op, n, [fill] * (2*n)
  end

  def [](i)
    @a[i + @n]
  end

  def []=(i, x)
    @a[i += @n] = x
    @a[i] = @op[@a[i<<1|0], @a[i<<1|1]] while (i >>= 1) > 0
  end

  def fold(l, r)
    l += @n
    r += @n
    x = y = @a[0]
    while l < r
      if (l & 1) == 1
        x = @op[x, @a[l]]
        l += 1
      end
      y = @op[@a[r += 1], y] if (r & 1) == 1
      l >>= 1
      r >>= 1
    end
    @op[x, y]
  end
end
