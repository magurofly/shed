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
