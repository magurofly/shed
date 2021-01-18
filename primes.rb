# 複数の素数を求めるような問題に高速に答えられる
class SPFPrime
	def initialize(max)
		@spf = (0..max).to_a
		sqmax = Math.sqrt(max).ceil
		4.step(sqmax, 2) { |i| @spf[i] = 2 }
		3.step(sqmax, 2) { |i| (i*i).step(max, i) { |j| @spf[j] = i if @spf[j] == j } if @spf[i] == i }
	end
	def prime?(n); @spf[n] == n; end
	def each; (2..max).each { |i| yield i if @spf[i] == i }; end
	def factorize(n); fs = []; while n > 1; f = @spf[n]; fs << f; n /= f; end; fs; end
end

# O(log n) で素数判定をする ただし n＜2^64
def prime?(n)
  return false if n <= 1
  return true if n == 2 or n == 7 or n == 61
  return false if (n & 1) == 0
  d = n - 1
  d >>= 1 while (d & 1) == 0
  [2, 7, 61].each do |a|
    t = d
    y = a.pow(t, n)
    while t != n - 1 and y != 1 and y != n - 1
      y = y * y % n
      t <<= 1
    end
    return false if y != n - 1 and (t & 1) == 0
  end
  true
end
