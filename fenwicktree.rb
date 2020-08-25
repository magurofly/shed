class BIT
  def initialize(size, id, &op)
    @n, @a, @id, @op = size, [id] * (size + 1), id, op
  end

  def query(i)
    s = @id
    while i > 0
      s = @op[s, @a[i]]
      i -= i & -i
    end
    s
  end

  def update(i, x)
    while i <= @n
      @a[i] = @op[@a[i], x]
      i += i & -i
    end
  end
end
