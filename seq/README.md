# 数列の計算

長さ $N$ の数列 $A_1, A_2, \ldots, A_N$ が与えられたとき、何かを計算する

ファイル名

- `(f)-pair-(g)`: 数列の非順序対に対する操作
  - $\underset{1 \le i \lt j \le N}{f} g(A_i, A_j)$

- `(f)-css-(g)`: 数列の連続部分列に対する操作
  - $\underset{1 \le L \le R \le N}{f} \underset{L \le i \le R}{g} A_i$