DEBUG = true
MOD = 10**9+7
YESNO = %w(Yes No)

def main
  
end

def int; gets.to_i; end
def ints; gets.split.map &:to_i; end
def array(&convert); gets.split.map(&convert); end
def string; gets.chomp; end
def rep(n, &b); Array.new(n, &b); end
def yes; puts YESNO[0]; end
def no; puts YESNO[1]; end
def yesno t; t ? yes : no; end
def zip(xs, *yss); Enumerator.new { |y| xs.zip(*yss) { |a| y.yield(*a) } }; end
def max(*xs); xs.max; end
def min(*xs); xs.max; end
def minmax(*xs); xs.minmax; end
def matrix(h, w, fill=nil); Array.new(h) { [fill] * w }; end
def debug(x); STDERR.puts x.inspect if DEBUG; end

def div_ceil(x, y); (x + y - 1) / y; end
def gcd(*xs); xs.inject(0) { |y, x| y.gcd(x) }; end
def factorial(n, mod); (2..n).inject(1) { |f, x| f * x % mod }; end
def cumsum(xs); ys = [0]; xs.each { |x| ys << x + ys[-1] }; ys; end
def cumdiff(ys); xs = []; xs.inject { |x, y| xs << (d = y - x); d }; end
def cumfold(ys, range); r = range.end; r -= 1 if range.exclusive_end?; ys[r] - ys[range.begin-1]; end
def mod_inv(x, mod); x.pow(mod-2, mod); end
def mod_div(x, y, mod); x * mod_inv(y, mod) % mod; end
def bitbrute(size, block); (1<<size).times(&block); end

main
