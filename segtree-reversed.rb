# 区間更新一点取得用のセグ木
class SegTreeReversed
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

  def get(i)
    x = @a[i += @n]
    x = @op[x, @a[i]] while (i >>= 1) > 0
    x
  end

  def put(l, r = l + 1, x)
    l += @n
    r += @n
    while l < r
      if (l & 1) == 1
        @a[l] = @op[x, @a[l]]
        l += 1
      end
      @a[r -= 1] = @op[@a[r], x] if (r & 1) == 1
      l >>= 1
      r >>= 1
    end
  end
end
