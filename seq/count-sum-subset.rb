# 部分集合の和の個数を数え上げる
def count_sum_subset(xs)
  count = Hash.new(0)
  count[0] = 1
  xs.each do |x|
    count.reverse_each do |y, c|
      count[x + y] += c
    end
  end
  count
end