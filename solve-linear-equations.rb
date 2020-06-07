require "matrix"

#----BEGIN FUNCTIONS----
# ---- util section ----

def pretty_complex(z)
	real, imag = z.real, z.imag
	if real.abs < Float::EPSILON
		return pretty_real(imag) + "i"
	elsif imag.abs < Float::EPSILON
		return pretty_real(real)
	else
		return z.inspect
	end
end

def pretty_real(x)
	return "0" if x.abs < Float::EPSILON

	if Rational === x
		return pretty_real(x.numerator) if 1 === x.denominator
	end

	return x.inspect
end

#----END FUNCTIONS----


print "Number of variables: "
n = gets.to_i

puts "Input equations (i.e. '3x + 2y = 1' is '3 2 1')"

coeffs = []
consts = []
n.times do |i|
	print "Equation (#{i+1}): "
	*coeff, const = gets.split.map(&:to_c)
	coeffs << coeff
	consts << const
end

puts "Solving..."

solution = Matrix[*coeffs].lup.solve(consts)
puts "Solution is [" + solution.to_a.map { |z| pretty_complex(z) }.join(", ") + "]"
