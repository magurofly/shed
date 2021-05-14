# mod p の原子根を全部求める O(p log p)
def primitive_roots(p)
  return [1] if p == 2
  visited = [false] * p
  (2 ... p).each do |r|
    roots = []
    x = r
    count = 1
    while x != 1
      roots << x if count.gcd(p - 1) == 1
      visited[x] = true
      x = x * r % p
      count += 1
    end
    return roots if count == p - 1
  end
end
