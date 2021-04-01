// モノイドかチェックする
template<class S, S (*op)(S, S), S (*e)(), S (*sample)()>
constexpr bool is_monoid(int iteration) {
  S id = e();
  // 単位元チェック
  for (int i = 0; i < iteration; i++) {
    S x = sample();
    if (op(x, id) != x || op(id, x) != x) return false;
  }
  // 結合則チェック
  for (int i = 0; i < iteration; i++) {
    S x = sample(), y = sample(), z = sample();
    if (op(x, op(y, z)) != op(op(x, y), z)) return false;
  }
  return true;
}

// 遅延セグ木に乗るかチェックする
template<class S, S (*op)(S, S), S (*e)(), class F, S (*mapping)(F, S), F (*composition)(F, F), F (*id)(), S (*sample_s)(), F (*sample_f)()>
constexpr bool is_lazysegable(int iteration) {
  if (!is_monoid<S, op, e>(iteration)) return false;
  if (!is_monoid<F, composition, id>(iteration)) return false;
  // 自己準同型チェック
  for (int i = 0; i < iteration; i++) {
    F f = sample_f();
    S x = sample_s(), y = sample_s();
    if (mapping(f, op(x, y)) != op(mapping(f, x), mapping(f, y))) return false;
  }
  return true;
}
