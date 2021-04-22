# 複数の素数を求めるような問題に高速に答えられる
class PrimeTable
	def initialize(n)
		@lpf = [nil] * (n + 1)
		@primes = [2]
		(2 .. n).step(2) do |d| @lpf[d] = 2 end
		(3 .. n).step(2) do |d|
			unless @lpf[d]
				@lpf[d] = d
				@primes << d
			end
			@primes.each do |p|
				break if p * d > n or p > @lpf[d]
				@lpf[p * d] = p
			end
		end
	end
	def prime?(n); @lpf[n] == n; end
	def each(&block); @primes.each(&block); end
	def factorize(n); fs = []; while n > 1; fs << (f = @lpf[n]); n /= f; end; fs; end
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

# O(sqrt n) で約数列挙をする
def factors(n)
  Enumerator.new do |y|
    k = 1
    while k * k < n
      if n % k == 0
        y << k
        y << n / k
      end
      k += 1
    end
    y << k if k * k == n
  end
end
