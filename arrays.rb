def sorted_uniq(xs)
  prev = nil
  xs.filter { |x| c = prev != x; prev = x; c }
end

def sorted_index(xs, y)
  xs.bsearch_index { |x| y <=> x }
end
