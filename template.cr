module Main

  
end

MOD = 998244353
# MOD = 10**9 + 7
INF = 1_000_000_000_i32
INF64 = 2_000_000_000_000_000_000_i64
YESNO = ["Yes", "No"]

def string; gets.to_s end
def strings; string.split end
def int; string.to_i end
def ints; strings.map { |s| s.to_i } end
def int64; string.to_i64 end
def ints64; strings.map { |s| s.to_i64 } end
def float; string.to_f64 end
def yes; puts YESNO[0] end
def no; puts YESNO[1] end
def yesno(c : Bool); puts c ? YESNO[0] : YESNO[1] end
# Enumerable#accumulate(0) で累積和を計算できる
def nCr(n, r) : Int64; x = 1_i64; (1..r).each { |i| x *= n + 1 - i; x //= i }; x end

struct Int
  def div_ceil(y : Int) : Int; (self + y - 1) // y end
  def pow(e : Int, m : Int) : Int; r, a = 1, self; while e > 0; r = r * a % m if e.odd?; a = a * a % m; e >>= 1; end; r end
  def inv(m : Int) : Int; n = self % m; raise "#{self} is divisible by #{m}" if n.zero?; p, q = {m, 0}, {n, 1}; while q[0] != 0; u = p[0] // q[0]; q, p = {p[0] - q[0] * u, p[1] - q[1] * u}, q end; g, x = p; g += m // g if g < 0; raise "inverse is not exist" if g != 1; x end
  def each_divisor; (1 .. self).each { |d| a = d * d; break if a > self; next if self % d != 0; yield(d); yield(self // d) if a != self } end
  def prime? : Bool; if self <= 1; false; elsif self > 100 && self < 4_759_123_141; self.miller_rabin_test([2, 7, 61]) else self.each_divisor { |x| return false if x != 1 && x != self }; true end end
  def [](bit); self >> bit & 1 end
  def miller_rabin_test(bases : Array(Int)) : Bool; return false if self <= 1; d = self - 1; d >>= d.trailing_zeros_count; bases.all? { |a| return self == a if self % a == 0; y = a.pow(t = d, self); while t != self - 1 && y != 1 && y != self - 1; y = y * y % self; t <<= 1 end; y == self - 1 || t.odd? } end
end

struct Range(B, E)
  def end_exclusive; excludes_end? ? self.end : self.end + 1 end
  def end_inclusive; excludes_end? ? self.end - 1 : self.end end
  def lower_bound(&block : B | E -> Bool); bsearch(&block) end
  def upper_bound(&block : B | E -> Bool); i = (-end_inclusive .. -self.begin).bsearch { |x| yield(-x) }; i ? -i : nil end
end

class Array(T)
  def compress(table = Hash(T, Int32).new); uniq.sort.each_with_index { |x, i| table[x] = i }; map { |x| table[x] } end
end

class Dsu
  @c : Array(Array(Int32))
  def initialize(n : Int32); @c = Array.new(n) { |i| [i] } end
  def merge(i : Int32, j : Int32) : Bool; x, y = @c[i], @c[j]; return false if x.same?(y); x, y = y, x if x.size < y.size; y.each { |v| @c[v] = x << v }; true end
  def same?(i : Int32, j : Int32) : Bool; @c[i].same?(@c[j]) end
  def [](i : Int32); @c[i] end
  def groups; @c.uniq { |x| x[0] } end
end
