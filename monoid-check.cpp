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
