=begin
# Usage
parser = AParer.new
parser.input = gets.chomp
puts parser.parse_xxx
=end

class AParser

  # write parser methods here

  # ----

  attr_reader :vars

  def initialize
    @vars = {}
  end

  def input=(input)
    @list = input.split
    @index = 0
  end

  def get?(token)
    t = @list[@index]
    if token === t
      @index += 1
      return yield t if block_given?
      return t
    end
    nil
  end

  def get
    token = @list[@index]
    @index += 1
    token
  end

  def consume(n)
    @index += n
  end

  def all
    i = @index
    v = yield
    @index = i unless v
    v
  end
end
