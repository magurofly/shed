class SparseTable
  def initialize(a, &op)
    n = a.size
    @op = op
    @table = Array.new(n) { |i| [a[i]] }
    (1 .. (n - 1).bit_length - 1).each do |k|
      (n - (1 << k) + 1).times do |i|
        @table[i][k] = op[@table[i][k - 1], @table[i + (1 << (k - 1))][k - 1]]
      end
    end
  end
  
  def prod(l, r)
    return @table[l][0] if l + 1 == r
    k = (r - l - 1).bit_length - 1
    @op[@table[l][k], @table[r - (1 << k)][k]]
  end
end
