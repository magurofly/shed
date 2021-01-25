# 最大長方形を計算する
# @param histogram 最後の要素は0である必要がある
def max_rect(histogram)
  rect = 0
  stack = []
  histogram.each_with_index do |x, i|
    if stack.empty?
      stack << [x, i]
    elsif stack[-1][0] < x
      stack << [x, i]
    elsif stack[-1][0] > x
      j = i
      while !stack.empty? and stack[-1][0] > x
        y, j = stack.pop
        rect = [rect, y * (i - j)].max
      end
      stack << [x, j]
    end
  end
  rect
end
