# (GF(2^n), ^, &) 上の行列で掃出し法をする（基底ベクトルを求める）
# @param rows 整数の配列（ビット単位で見ると行列）
def gf2_basis(rows)
  basis = []
  rows.each do |e|
    e = basis.reduce(e) { |e, b| [e, e ^ b].min }
    basis << e if e > 0
  end
  basis
end
