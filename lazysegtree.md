# 遅延セグ木（定型）

```C++
#include <atcoder/lazysegtree>
using namespace atcoder;
```

## Range Affine Range Sum
```C++
using S = pair<ll, int>; // (値, 区間の長さ)
S op(S x, S y) {
  return { (x.first + y.first), (x.second + y.second) };
}
S e() {
  return { 0, 0 };
}

using F = pair<ll, ll>; // (掛ける値, 足す値)
F composition(F f, F g) {
  return { (f.first * g.first), (f.second + f.first * g.second) };
}
F id() {
  return {1, 0};
}

S mapping(F f, S x) {
  return { (f.first * x.first + f.second * x.second), x.second };
}


lazy_segtree<S, op, e, F, mapping, composition, id> seg;
```

## Range Update Range Min
```C++
using S = pair<ll, int>; // (値, 区間の長さ)
S op(S x, S y) {
  return { min(x.first, y.first), (x.second + y.second) };
}
S e() {
  return { INF, 0 };
}

using F = ll; // 更新する値
F composition(F f, F g) {
  return (f == id() ? g : f);
}
F id() {
  return -INF;
}

S mapping(F f, S x) {
  if (f == id()) return x;
  return { f, x.second };
}


lazy_segtree<S, op, e, F, mapping, composition, id> seg;
```
