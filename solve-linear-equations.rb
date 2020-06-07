require "matrix"

print "Number of variables: "
n = gets.to_i

puts "Input equations (i.e. '3x + 2y = 1' is '3 2 1')"

coeffs = []
consts = []
n.times do |i|
	print "Equation (#{i+1}): "
	*coeff, const = gets.split.map(&:to_f)
	coeffs << coeff
	consts << const
end

puts "Solving..."

p Matrix[*coeffs].lup.solve(consts)