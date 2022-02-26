# https://wandbox.org/permlink/dXdJBcctPde3843G

index_start = 1 # 0 にすると 0-index
cap_max = 10**9

STDERR.print "level = "
level = gets.to_i
raise "level must be >= 1" if level < 1

N = 2 * level
STDERR.puts "N = #{N}"

arcs = []

# level 1
s = N / 2 - 1
t = N / 2
arcs << [s, t, 1]

(level - 1).times do |i|
  a, b = s - 1, t + 1
  cap = [cap_max, 2**i].min
  arcs << [a, s, cap] << [a, t, cap] << [s, b, cap] << [t, b, cap]
  s, t = a, b
end

STDERR.puts "s = #{s}"
STDERR.puts "t = #{t}"

puts "#{N} #{arcs.size}"
arcs.each do |(u, v, c)|
  puts "#{u + index_start} #{v + index_start} #{c}"
end
