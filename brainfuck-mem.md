# Brainfuckコード生成器

[ソース](https://github.com/magurofly/shed/blob/master/brainfuck-mem.rb)

使い方

```ruby
require "./brainfuck-mem.rb"

b = BrainMem.new

# メモリを確保
x = b.alloc
y = b.alloc
z = b.alloc

# メモリを解放
z.free

# 代入
x.set 2
y.zero

# 加算
y.add 3
x.add y

# 出力（1桁）
y.putdigit

# コードの生成
puts b.bf.to_s

# 実行（デバッグ）
b.bf.run_dump
```

出力されたコード

```brainfuck
++>[-]+++>[-]<[>+>+<<-]>>[<<+>>-]<[-<<+>>]<++++++++++++++++++++++++++++++++++++++++++++++++.------------------------------------------------
```
