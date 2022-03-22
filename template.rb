main = -> {
  
}

DEBUG = true
MOD = 10**9+7
$yesno = %w(No Yes)
YesNo = %w(No Yes)
YESNO = %w(NO YES)
INF = 10**18

require "set"
require "prime"

def int; gets.to_s.to_i end
def ints; gets.to_s.split.map { |s| s.to_i } end
def int1s; gets.to_s.split.map { |s| s.to_i - 1 } end
def float; gets.to_s.to_f end
def floats; gets.to_s.split.map { |s| s.to_f } end
def array_of(&convert); gets.to_s.split.map(&convert) end
def string; gets.to_s.chomp end
def strings; gets.to_s.split end
def rep(n, &b); Array.new(n, &b) end
def yes; puts $yesno[1] end
def no; puts $yesno[0] end
def yesno t; puts $yesno[t] end
def YesNo t; puts YesNo[t] end
def YESNO t; puts YESNO[t] end
def zip(xs, *yss); Enumerator.new { |y| xs.zip(*yss) { |a| y.yield(*a) } } end
def max(*xs, &block); block_given? ? xs.max_by(&block) : xs.max end
def min(*xs, &block); block_given? ? xs.min_by(&block) : xs.min end
def minmax(*xs, &block); block_given? ? xs.minmax_by(&block) : xs.minmax end
def gcd(*xs); xs.inject(0, :gcd) end
def matrix(h, w, fill=nil, &block); return Array.new(h) { Array.new(w, &block) } if block_given?; Array.new(h) { [fill] * w } end
def debug(x = nil); STDERR.puts (block_given? ? yield(x) : x).inspect if DEBUG; x end
def debug_grid(grid, width = 1); grid.each { |row| STDERR.puts row.map { |x| x.inspect.ljust(width) }.join("") } if DEBUG; grid end
def if_debug; yield if DEBUG end

module Boolean
  def coerce(other); [other, to_i] end
  def +@; to_i end
  def to_int; to_i end
  def *(other); to_i * other end
end

class TrueClass
  include Boolean
  def to_i; 1 end
end

class FalseClass
  include Boolean
  def to_i; 0 end
end

class Integer
  def div_ceil(y); (self + y - 1) / y end
  def mod_inv(mod = MOD); pow(mod-2, mod) end
  def mod_div(y, mod = MOD); self * y.mod_inv(mod) % mod end
  def mod_nCr(r, mod = MOD); x = y = 1; (1..r).each { |i| x = x * (self + 1 - i) % mod; y = y * i % mod }; x.mod_div(y, mod); end
  def factorial(mod = MOD); (2..self).inject(1) { |f, x| f * x % mod } end
  def popcount; x = self; c = 0; while x > 0; c += 1 if x & 1 == 1; x >>= 1 end; c end #TODO: faster
  def bitbrute(&block); (1<<self).times(&block) end
  def nCr(r); x = 1; (1..r).each { |i| x *= self + 1 - i; x /= i }; x; end
  def each_divisor; return Enumerator.new { |y| each_divisor { |k| y << k } } unless block_given?; k = 1; while k * k < self; if self % k == 0; yield k; yield self / k end; k += 1; end; yield k if k * k == self end
  def divisors; each_divisor.to_a end
end

class Range
  def end_open; exclude_end? ? self.end : self.end + 1 end
  def end_close; exclude_end? ? self.end - 1 : self.end end
  def upper_bound; ac, wa = self.begin, self.end_open; while wa - ac > 1; if yield((wj = (ac + wa) / 2)); ac = wj else wa = wj end; end; yield(ac) ? ac : nil end
  def lower_bound; ac, wa = self.end_open, self.begin; while ac - wa > 1; if yield((wj = (ac + wa) / 2)); ac = wj else wa = wj end; end; yield(ac) ? ac : nil end
  def shakutori(r2, &pred); Enumerator.new { |y| j, r = r2.begin, r2.end_open; each { |i| j += 1 while j + 1 < r and pred[i, j+1]; y.yield(i, j) } }; end
  def widest(&block); Enumerator.new { |y| j, n = self.begin, self.end_open; each { |i| j += 1 while j < n and block[i, j]; y.yield(i, j) if block[i, j] } } end
end

class Array
  def power(&block); (0 ... 1 << size).each(&block) end
  def sorted_merge(other); a = []; i = j = 0; n, m = size, other.size; if j < m and other[j] < self[i]; a << other[j]; j += 1 else; a << self[i]; i += 1 end while i < n; a.push(*other[j..-1]) if j < m; a end
  def upper_bound; ac, wa = 0, size; while wa - ac > 1; if yield(self[(wj = (ac + wa) / 2)]); ac = wj else; wa = wj end; end; ac end
  def lower_bound; ac, wa = size, 0; while wa - ac > 1; if yield(self[(wj = (ac + wa) / 2)]); ac = wj else; wa = wj end; end; ac end
  def cum(*xs, &op); a = []; a << xs[0] if xs.size > 0; a << x = self[0]; (1...size).each { |i| a << x = op[x, self[i]] }; a end
  def cumdiff(range); self[range.end_open] - self[range.begin]; end
  def compress(kinds = uniq.sort!); map { |x| kinds.bsearch_index { _1 >= x } }; end
end

module Enumerable
  def sorted_uniq; x = nil; filter { |y| c = x === y; x = y; !c } end
  def cumsum; ys = [0]; each { |x| ys << x + ys[-1] }; ys end
end

main[]
