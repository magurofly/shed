DEBUG = false
MOD = 10**9+7
YESNO = %w(Yes No)

def main
  
end

def int; gets.to_i; end
def ints; gets.split.map &:to_i; end
def float; gets.to_f; end
def floats; gets.split.map &:to_f; end
def array_of(&convert); gets.split.map(&convert); end
def string; gets.chomp; end
def rep(n, &b); Array.new(n, &b); end
def yes; puts YESNO[0]; end
def no; puts YESNO[1]; end
def yesno t; t ? yes : no; end
def zip(xs, *yss); Enumerator.new { |y| xs.zip(*yss) { |a| y.yield(*a) } }; end
def max(*xs, &block); block_given? ? xs.max_by(&block) : xs.max; end
def min(*xs, &block); block_given? ? xs.min_by(&block) : xs.min; end
def minmax(*xs, &block); block_given? ? xs.minmax_by(&block) : xs.minmax; end
def gcd(*xs); xs.inject(0) { |y, x| y.gcd(x) }; end
def matrix(h, w, fill=nil, &block); return Array.new(h) { Array.new(w, &block) } if block_given?; Array.new(h) { [fill] * w }; end
def debug(x); if DEBUG; STDERR.puts (block_given? ? yield(x) : x).inspect; end; x; end
def debug_grid(grid, width = 1); grid.each { |row| STDERR.puts row.map { |x| x.inspect.ljust(width) }.join("") } if DEBUG; grid; end
def if_debug; yield if DEBUG; end

class Integer
  def div_ceil(y); (self + y - 1) / y; end
  def mod_inv(mod = MOD); pow(mod-2, mod); end
  def mod_div(y, mod = MOD); self * mod_inv(y, mod) % mod; end
  def factorial(mod = MOD); (2..self).inject(1) { |f, x| f * x % mod }; end
  def popcount; x = self; c = 0; while x > 0; c += 1 if x & 1 == 1; x >>= 1; end; c; end #TODO: faster
  def bitbrute(&block); (1<<self).times(&block); end
end

class Range
  def end_open; exclude_end? ? self.end : self.end + 1; end
  def end_close; exclude_end? ? self.end - 1 : self.end; end
  def upper_bound; ac, wa = self.begin, self.end_open; while ac + 1 < wa; wj = (ac + wa) / 2; if yield(wj); ac = wj; else; wa = wj; end; end; ac; end
  def lower_bound; ac, wa = self.end_open, self.begin; while ac > wa + 1; wj = (ac + wa) / 2; if yield(wj); ac = wj; else; wa = wj; end; end; ac; end
end

class Array
  def sorted_uniq; x = nil; xs.filter { |y| c = x === y; x = y; !c }; end
  def cumsum; ys = [0]; each { |x| ys << x + ys[-1] }; ys; end
  def cumdiff; xs = []; inject { |x, y| xs << (d = y - x); d }; end
  def cumfold(range); r = range.end; r -= 1 if range.exclusive_end?; self[r] - self[range.begin-1]; end
end

class UnionFind
  def initialize(size); @p, @r = size.times.to_a, [1] * size; end
  def merge(i, j); k, l = leader(i), leader(j); return if k == l; k, l = l, k if @r[k] < @r[l]; @p[l] = k; @r[k] += @r[l]; end
  def size(i); @r[leader(i)]; end
  def same?(i, j); leader(i) == leader(j); end
  def leader(i); j = i; until i == @p[i]; j, i = i, @p[j] = @p[i]; end; i; end
end

class Factorials
  def initialize(limit, mod = MOD); @fac, @fin, @inv, @mod = [1, 1], [1, 1], [nil, 1], mod; (2..limit).each { |i| @fac[i] = @fac[i-1] * i % mod; @inv[i] = mod - @inv[mod % i] * (mod / i) % mod; @fin[i] = @fin[i-1] * @inv[i] % mod }; end
  def fact(n); @fac[n]; end
  def comb(n, k); perm(n, k) * @fin[k] % @mod; end
  def perm(n, k); @fac[n] * @fin[n-k] % @mod; end
end

main
