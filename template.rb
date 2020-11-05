DEBUG = true
MOD = 10**9+7

def main


end

def int; gets.to_i; end
def ints; gets.split.map &:to_i; end
def string; gets.chomp; end
def rep(n, &b); n.times.map(&b); end
def yes; puts "Yes"; end
def no; puts "No"; end
def yesno t; t ? Yes : No; end
def debug(x); STDERR.puts x.inspect if DEBUG; end

def factorial(n, mod); (2..n).inject(1) { |f, x| f * x % mod }; end
def cumsum(xs); ys = [0]; xs.each { |x| ys << x + ys[-1] }; ys; end
def cumdiff(ys); xs = []; xs.inject { |x, y| xs << (d = y - x); d }; end

main
