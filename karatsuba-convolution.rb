def convolution_karatsuba(a, b)
  n, m = a.size, b.size
  return convolution_naive(a, b) if n <= 60 or m <= 60
  l = [n, m].min / 2
  al, ah = a[0 ... l], a[l ... n]
  bl, bh = b[0 ... l], b[l ... m]
  cl = convolution_karatsuba(al, bl)
  ch = convolution_karatsuba(ah, bh)
  ans = [0] * (n + m - 1)
  cl.each_with_index do |z, k|
    ans[k] += z
    ans[l + k] += z
  end
  ch.each_with_index do |z, k|
    ans[l + k] += z
    ans[l * 2 + k] += z
  end
  d1 = [0] * [l, n - l].max
  al.each_with_index do |x, i|
    d1[i] -= x
  end
  ah.each_with_index do |x, i|
    d1[i] += x
  end
  d2 = [0] * [l, m - l].max
  bl.each_with_index do |y, j|
    d2[j] -= y
  end
  bh.each_with_index do |y, j|
    d2[j] += y
  end
  d = convolution_karatsuba(d1, d2)
  d.each_with_index do |z, k|
    ans[l + k] -= z
  end
  ans
end

def convolution_naive(a, b)
  n, m = a.size, b.size
  return convolution_naive(b, a) if n < m
  ans = [0] * (n + m - 1)
  i = 0
  while i < n
    j = 0
    while j < m
      ans[i + j] += a[i] * b[j]
      j += 1
    end
    i += 1
  end
  ans
end
