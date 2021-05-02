# 余りと法のペアの配列から元の値と法を復元する（Rubyなので多倍長対応）
# @return [x, m]
def crt(pairs)
  r = 0
  mM = 1
  pairs.each do |(bi, mi)|
    val = {}
    d, p, q = extgcd(mM, mi, 0, 0)
    return [0, -1] if (bi - r) % d != 0
    tmp = (bi - r) / d * p % (mi / d)
    r += mM * tmp
    mM *= mi / d
  end

  [r % mM, mM]
end

def extgcd(a, b, p, q)
  return [a, 1, 0] if b == 0
  d, q, p = extgcd(b, a % b, q, p)
  q -= a / b * p
  [d, p, q]
end
