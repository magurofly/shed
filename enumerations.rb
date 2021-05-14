class Factorial
  def initialize(max, mod)
    @mod = mod
    @fac = [1, 1]
    @fin = [1, 1]
    @inv = [nil, 1]
    (2 .. max).each do |i|
      @fac[i] = @fac[i - 1] * i % mod
      @inv[i] = mod - @inv[mod % i] * (mod / i) % mod
      @fin[i] = @fin[i - 1] * @inv[i] % mod
    end
  end
  
  def fact(n)
    @fac[n]
  end
  
  def comb(n, k)
    return 0 if n < k or n < 0 or k < 0
    @fac[n] * @fin[k] % @mod * @fin[n - k] % @mod
  end
end
