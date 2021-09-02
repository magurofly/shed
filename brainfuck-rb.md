# 基本的な使い方

## インスタンスの生成
`bf = Brainfuck.new`

## Brainfuckコードの追加
`bf << "[+.]"`

## Brainfuckコードの実行（入力は省略可能、返り値は出力）
`bf.run("")`

## Brainfuckコードを実行した後に内部状態を表示
`bf.run_dump("")`

## これまでに追加されたBrainfuckコード
`bf.to_s`

## コードを消去する
`bf.clear`

# ユーティリティ

## `add(n, shift=1)`
`mem[ptr] += n`

作業領域として`mem[ptr + shift]`を使う

## `sub(n, shift=1)`
`mem[ptr] += n`

作業領域として`mem[ptr + shift]`を使う

## `move(n)`
`ptr += n`

`n`が負なら左に移動する

## `repeat { ... }`
`while mem[ptr] > 0; mem[ptr] -= 1; ...; end`

`...`は`bf.add`など

## `times(n, shift=1) { ... }`
`...`を`n`回実行する

作業領域として`mem[ptr+shift]`を使う

## 入力

### `getchar`
1文字読み込み、文字コードを`mem[ptr]`に代入する

### `getdigit`
1文字読み込み、数値として解釈して`mem[ptr]`に代入する

## 出力

### `putchar`
`mem[ptr]`が文字コードの文字を出力する

### `putdigit`
`mem[ptr]`をひと桁の数値として出力する

`mem[ptr] >= 10`だと壊れる

# コード
```ruby:bf.rb
class Brainfuck

  attr_accessor :program

  def initialize
    @program = ""
  end

  def to_s
    @program
  end

  def <<(str)
    @program << str
  end

  def make(input = "")
    BrainfuckEnv.new(@program, input)
  end

  def run(input = "", count = 10000)
    env = make(input)
    env.run(count)
    env.output
  end

  def run_dump(input = "", count = 10000)
    env = make(input)
    env.run(count)
    env.dump
    env.output
  end

  def clear
    @program = ""
  end

  # 以下、util

  # mem[ptr] += n
  # assert mem[ptr+shift*x] == 0
  def add(n = 1, shift = 1)
    if n < 0
      sub -n

      return
    end

    if n >= 64
      right shift
      add 8 * (n / 64), shift
      while_positive do
        sub 1
        left shift
        add 8
        right shift
      end
      left shift
      n %= 64
    end
    
    @program << ?+ * n

    return
  end

  # mem[ptr] -= n
  def sub(n = 1, shift = 1)
    if n < 0
      add -n
      
      return
    end

    if n >= 64
      right shift
      add 8 * (n / 64), shift
      while_positive do
        sub 1
        left shift
        sub 8
        right shift
      end
      left shift
      n %= 64
    end
    
    @program << ?- * n

    return
  end

  # ptr -= n
  def left(n = 1)
    @program << ?< * n

    return
  end

  # ptr += n
  def right(n)
    @program << ?> * n

    return
  end

  # yield while mem[ptr] != 0
  def while_positive
    @program << ?[
    yield
    @program << ?]

    return
  end
  
  # ptr += n
  def shift(n)
    if n > 0
      right n
    elsif n < 0
      left -n
    end

    return
  end

  # mem[ptr].times { mem[ptr] -= 1; yield }
  def repeat(shift = 1)
    @program << "[-"
    yield
    @program << "]"

    return
  end

  # ptr += 1; n.times { yield }; ptr -= 1
  # assert mem[ptr+shift] == 0
  def times(n = 1, shift = 1)
    shift shift
    add(n, shift)
    repeat do
      shift -shift
      yield
      shift shift
    end
    shift -shift

    return
  end

  # -- 入出力関係 --

  # mem[ptr, n] = read(n).each_char.map(&:ord)
  def getchar(n = 1)
    @program << ?, + ">," * (n - 1)

    return
  end

  # print mem[ptr, n].map(&:chr).join
  def putchar(n = 1)
    @program << ?. + ">." * (n - 1)

    return
  end

  # mem[ptr] = read(1).to_i
  def getdigit
    getchar
    sub ?0.ord
  end

  # print mem[ptr] if mem[ptr] < 10
  def putdigit
    add ?0.ord
    putchar
    sub ?0.ord
  end
end

class BrainfuckEnv
  MEM = 10000

  attr_accessor :program, :pc, :input, :read, :mem, :ptr, :output

  def initialize(program, input)
    @program = program
    @pc = 0
    @input = input
    @read = 0
    @mem = [0] * MEM
    @ptr = 0
    @ptr_max = 0
    @output = ""
  end

  def run(count = 100000)
    i = 0
    while i < count
      break unless step
      i += 1
    end

    if i >= count
      STDERR.puts "Brainfuck: program steps exceeded #{count}"
    end
  end

  def step
    return false if @pc >= @program.size
    case @program[@pc]
    when ?+
      @mem[@ptr] += 1
      @mem[@ptr] %= 256
      @pc += 1
    when ?-
      @mem[@ptr] -= 1
      @mem[@ptr] %= 256
      @pc += 1
    when ?<
      raise "Brainfuck: negative pointer" if @ptr <= 0
      @ptr -= 1
      @pc += 1
    when ?>
      raise "Brainfuck: memory limit exceeded" if @ptr >= MEM
      @ptr += 1
      @ptr_max = @ptr if @ptr_max < @ptr
      @pc += 1
    when ?.
      @output << @mem[@ptr].chr
      @pc += 1
    when ?,
      if @read < @input.size
        @mem[@ptr] = @input[@read].ord
        @read += 1
      else
        @mem[@ptr] = 255
      end
      @pc += 1
    when ?[
      if @mem[@ptr] == 0
        depth = 1
        index = @pc
        while depth > 0 and index + 1 < @program.size
          index += 1
          case @program[index]
          when ?[
            depth += 1
          when ?]
            depth -= 1
          end
        end
        raise "Brainfuck: expected ]" if depth > 0
        @pc = index
      end
      @pc += 1
    when ?]
      if @mem[@ptr] != 0
        depth = -1
        index = @pc
        while depth < 0 and index > 0
          index -= 1
          case @program[index]
          when ?[
            depth += 1
          when ?]
            depth -= 1
          end
        end
        if depth == 0
          @pc = index
        else
          # 対応するカッコがなければ先頭へ
          @pc = -1
        end
      end
      @pc += 1
    else
      @pc += 1
    end
    true
  end

  def dump(out = STDERR)
    program = "\e[m#{@program[0, @pc]}\e[31m@\e[m#{@program[@pc..-1]}".lines.join("          \e[31m|\e[m")
    input = "\e[m#{@input[0, @read]}\e[31m@\e[m#{@input[@read..-1]}".lines.join("          \e[31m|\e[m")
    output = @output.lines.join("          \e[31m|\e[m")
    out.puts <<-EOT
program:  \e[31m{#{program}\e[31m}\e[m
input:    \e[31m{#{input}\e[31m}\e[m
output:   \e[31m{\e[m#{output}\e[31m}\e[m
memory:
    EOT
    memory_rows = (0 ... (@ptr_max + 1 + 15) / 16).map { |i| ["    %02X: " % (i * 16), @mem[i * 16, 16].map { |d| "%02X" % d }] }
    memrow = memory_rows[@ptr / 16][1]
    memrow[@ptr % 16] = "\e[44;37m" + memrow[@ptr % 16]
    memrow[@ptr % 16] += "\e[m"
    out.puts memory_rows.map { |prefix, mem| prefix + mem.join(" ") }
  end

  def dump_run(wait = STDIN, out = STDERR, interval = 0.1)
    out.puts "Enter to stop"
    running = true
    t = Thread.fork do
      while running
        break unless step
        dump
        sleep interval if interval
      end
    end
    wait.gets
    running = false
    dump
  end
end
```
