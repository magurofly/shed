# 複数の素数を求めるような問題に高速に答えられる
class SPFPrime
	def initialize(max)
		@spf = (0..max).to_a
		sqmax = Math.sqrt(max).ceil
		4.step(sqmax, 2) { |i| @spf[i] = 2 }
		3.step(sqmax, 2) { |i| (i*i).step(max, i) { |j| @spf[j] = i if @spf[j] == j } if @spf[i] == i }
	end
	def each; (2..max).each { |i| yield i if @spf[i] == i }; end
	def factorize(n); fs = []; while n > 1; f = @spf[n]; fs << f; n /= f; end; fs; end
end
