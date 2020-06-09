# ascending heap
class Heap
  def initialize(comparator = nil)
    @comparator = comparator || ->(a, b) { return a <=> b }
    @array = []
  end
  
  def push(*xs)
  	xs.each do |x|
	  	i = @array.size
	  	j = i - 1 >> 1
	  	@array << x
	  	until i == 0 or @comparator[@array[j], @array[i]] < 0
	  		@array[j], @array[i] = @array[i], @array[j]
	  		i, j = j, j - 1 >> 1
	  	end
	end
	self
  end

  alias_method :<<, :push

  def shift
  	ret, x = @array.first, @array.pop
  	i, n = 0, @array.size
  	loop do
  		j = i << 1 | 1
  		return ret if j >= n
  		a = @comparator[@array[i], @array[j]]
  		if (k = j + 1) < n
  			if a > 0
  				j = k if @comparator[@array[j], @array[k]] > 0
  			elsif @comparator[@array[i], @array[k]] > 0
  				j = k
  			else
  				break
  			end
  			@array[j], @array[i] = @array[i], @array[j]
  			i = j
  		else
  			if a > 0
  				@array[j], @array[i] = @array[i], @array[j]
  				i = j
  			end
  			break
  		end
  	end
  	ret
  end
end
