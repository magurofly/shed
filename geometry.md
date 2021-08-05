# 格子多角形の面積
格子多角形の面積はピックの定理を使うと求まる。変形をすると直接座標から求める。

```Ruby
def polygon_area(pos)
  (0 ... pos.size).sum { |i| pos[i][0] * pos[i - 1][1] - pos[i][1] * pos[i - 1][0] }.abs / 2
end
```

# 図形の点の順序

点が時計回りに並んでいるか、反時計回りに並んでいるか確認する。

```Ruby
def is_clockwise(pos)
  (0 ... pos.size).sum { |i| pos[i - 1][0] * pos[i][1] - pos[i][0] * pos[i - 1][1] } < 0
end
```

# 2 点を通る直線と点の距離

点 `p1`, `p2` を通る直線と点 `q` の距離を求める

- $ \frac { |(p_2 - p_1) \times (q - p_1)| } { | p_2 - p_1 | } $
