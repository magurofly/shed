# coding: UTF-8

=begin
brainfuck.rb

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

## 計算

### `zero`
`mem[ptr] = 0`

### `add(n, s=1)`
`mem[ptr] += n`

作業領域として`mem[ptr + s]`を使う

### `sub(n, s=1)`
`mem[ptr] += n`

作業領域として`mem[ptr + s]`を使う

## メモリ

### `shift(n)`
`ptr += n`

`n`が正なら右に、負なら左に移動する

### `move(s)`
`mem[ptr], mem[ptr+s] = 0, mem[ptr]`

### `copy(s)`
`mem[ptr+s] = mem[ptr]`

作業領域として`mem[ptr + s * 2]`を使う

## 制御

### `while_positive { ... }`
`while mem[ptr] > 0; ...; end`

### `repeat { ... }`
`while mem[ptr] > 0; mem[ptr] -= 1; ...; end`

`...`は`bf.add`など

### `times(n, s=1) { ... }`
`...`を`n`回実行する

作業領域として`mem[ptr+s]`を使う


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

=end
class Brainfuck
  MEM = 10000
  DEFAULT_LIMIT = 100000

  attr_accessor :program, :indent

  def initialize
    @program = ""
    @indent = 0
    @newline = true
  end

  def to_s
    @program
  end

  def <<(str)
    @newline = false
    @program << str
  end

  def make(input = "")
    Env.new(@program, input)
  end

  def run(input = "", count = DEFAULT_LIMIT)
    env = make(input)
    env.run(count)
    env.output
  end

  def run_dump(input = "", count = DEFAULT_LIMIT)
    env = make(input)
    env.run(count)
    env.dump
    env.output
  end

  def newline
    self << ?\n << "  " * @indent
    @newline = true
  end

  def comment(text = "")
    newline unless @newline
    self << "# " << escape(text)
    newline
  end

  def escape(text)
    text.gsub(/\+/, '＋').gsub(/-/, 'ー').gsub(/</, '＜').gsub(/>/, '＞').gsub(/\[/, '［').gsub(/\]/, '］').gsub(/\./, '．').gsub(/,/, '，')
  end

  def clear
    @program = ""
  end

  # 以下、util

  # -- 計算 --

  # mem[ptr] = 0
  def zero
    self << "[-]"

    return
  end

  # mem[ptr] += n
  # assert mem[ptr+shift*x] == 0
  def add(n = 1)
    if n < 0
      sub -n

      return
    end

    self << ?+ * n

    return
  end

  # mem[ptr] -= n
  def sub(n = 1, s = 1)
    if n < 0
      add -n
      
      return
    end
    
    self << ?- * n

    return
  end

  # -- メモリ --
  
  # ptr += n
  def shift(n)
    if n > 0
      self << ?> * n
    elsif n < 0
      self << ?< * -n
    end

    return
  end

  # mem[ptr], mem[ptr + s] = 0, mem[ptr]
  def move(s = 1)
    repeat do
      shift s
      add 1
      shift -s
    end
  end

  # mem[ptr], mem[ptr + s], mem[ptr + t] = 0, mem[ptr], mem[ptr]
  def move2(s = 1, t = 2)
    repeat do
      shift s
      add 1
      shift t - s
      add 1
      shift -t
    end
  end

  # mem[ptr + s] = mem[ptr]
  # assert mem[ptr + s * 2] == 0
  def copy(to = 1, temp = 2)
    repeat do
      shift to
      add 1
      shift temp - to
      add 1
      shift -temp
    end
    shift temp
    move -temp
  end

  # -- 制御 --

  # yield while mem[ptr] != 0
  def while_positive
    self << ?[
    yield
    self << ?]

    return
  end

  # yield if mem[ptr] != 0; mem[ptr] = 0
  def if_positive
    while_positive do
      yield
      zero
    end
  end

  # mem[ptr].times { mem[ptr] -= 1; yield }
  def repeat
    self << "[-"
    yield
    self << "]"

    return
  end

  # ptr += 1; n.times { yield }; ptr -= 1
  # assert mem[ptr+s] == 0
  def times(n = 1, s = 1)
    shift s
    add(n, s)
    repeat do
      shift -s
      yield
      shift s
    end
    shift -s

    return
  end

  # -- 入力 --

  # mem[ptr, n] = read(n).each_char.map(&:ord)
  def getchar(n = 1)
    self << ?, + ">," * (n - 1)

    return
  end

  # mem[ptr] = read(1).to_i
  def getdigit
    getchar
    sub ?0.ord
  end

  # -- 出力 --

  # print mem[ptr, n].map(&:chr).join
  def putchar(n = 1)
    self << ?. + ">." * (n - 1)

    return
  end

  # print mem[ptr] if mem[ptr] < 10
  def putdigit
    add ?0.ord
    putchar
    sub ?0.ord
  end

  class Env
  
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
  
    def run(count = DEFAULT_LIMIT)
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
end

class BrainMem
  attr_reader :bf, :ptr, :mem
  def initialize(verbose = false)
    @bf = Brainfuck.new
    @mem = [true] * 10000
    @ptr = 0
    @verbose = verbose
  end

  def exec(&block)
    self.instance_exec(&block)
  end

  class Ptr
    attr_reader :ptr, :size
    def initialize(ptr, size, bm = nil)
      @ptr, @size, @bm = ptr, size, bm
    end

    def to_s
      if @size != 1
        "$(#{@ptr}:#{@size})"
      else
        "$#{@ptr}"
      end
    end

    def free
      @bm.free(self)
    end

    def move_to(dst)
      @bm.move(dst, self)
    end

    def copy_to(dst, tmp = nil)
      @bm.copy(dst, self, tmp)
    end

    %i(move copy zero getchar getdigit putchar putdigit set add sub).each do |name|
      define_method(name) { |*args| @bm.method(name).call(self, *args) }
    end

    def times(&block)
      @bm.times(self, &block)
    end
  end

  def alloc(size = 1, base = @ptr)
    ptr = find_nearest_free(size, base)
    raise "Brainfuck: failed to alloc" unless ptr
    # STDERR.puts "alloc #{ptr}:#{size}"
    size.times do |i|
      @mem[ptr + i] = false
    end
    Ptr.new(ptr, size, self)
  end

  def free(ptr)
    ptr, size = ptr.ptr, ptr.size
    # STDERR.puts "free #{ptr}:#{size}"
    # go_to ptr
    # _zero ptr
    size.times do |i|
      @mem[ptr + i] = true
    end
  end

  def find_nearest_free(size = 1, base = @ptr)
    i = base.upto(9999 - size + 1).find { |ptr| (0 ... size).all? { |offset| @mem[ptr + offset] } }
    if (i2 = (base - size + 1).downto(0).find { |ptr| (0 ... size).all? { |offset| @mem[ptr + offset] } })
      i = i2 if not i or (base - i).abs > (base - i2).abs
    end
    i
  end

  def alloc_tmp(size = 1, base = @ptr)
    ptr = alloc(size, base)
    ret = yield(ptr)
    free(ptr)
    ret
  end

  def alloc_tmps(count, size = 1, base = @ptr)
    ptrs = (0 ... count).map { alloc(size, base) }
    ret = yield(*ptrs)
    ptrs.each { |ptr| free(ptr) }
    ret
  end

  def go_tmp
    ptr = @ptr
    ret = yield
    go_to ptr
    ret
  end

  def go_by(size)
    @ptr += size
    if size < 0
      @bf << ?< * -size
    elsif size > 0
      @bf << ?> * size
    end
  end

  def go_to(to, offset = 0, from = @ptr)
    to = to.ptr if to.is_a? Ptr
    go_by(to - from + offset)
  end

  def move(dst, src)
    @bf.comment "#{dst} = move(#{src})" if @verbose
    _move(dst, src)
  end

  def _move(dst, src)
    go_to src
    @bf << ?[
      go_to dst
      @bf << ?+
      go_to src
      @bf << ?-
    @bf << ?]
  end

  def zero(dst)
    @bf.comment "#{dst} = 0" if @verbose
    _zero(dst)
  end

  def _zero(dst)
    go_to dst
    @bf << "[-]"
  end

  def copy(dst, src, tmp = nil)
    @bf.comment "#{dst} = #{src}" if @verbose
    _copy(dst, src, tmp)
  end

  def _copy(dst, src, tmp = nil)
    return alloc_tmp { |ptr| _copy(dst, src, ptr) } unless tmp
    _zero(dst)
    go_to src
    @bf << ?[
      go_to dst
      @bf << ?+
      go_to tmp
      @bf << ?+
      go_to src
      @bf << ?-
    @bf << ?]
    _move(src, tmp)
  end

  def set(dst, src, tmp = nil)
    case src
    when Integer
      @bf.comment "#{dst} = #{src}" if @verbose
      _addsub_const ?+, dst, src
    when String
      @bf.comment "#{dst} = #{src.inspect}.ord" if @verbose
      _addsub_const ?+, dst, src.ord
    when Ptr
      copy(dst, src, tmp)
    end
  end

  def _addsub_const(op, dst, val)
    n = 5
    if val >= n**2
      alloc_tmp do |tmp|
        _addsub_const(?+, tmp, val / n**2 * n)
        go_to tmp
        @bf.repeat do
          go_to dst
          _addsub_const(op, dst, n)
          go_to tmp
        end
      end
      val %= n**2
    end
    go_to dst
    @bf << op * val
  end

  def _add(dst, src, scale = 1)
    go_to src
    @bf << "[-"
      go_to dst
      @bf << ?+ * scale
      go_to src
    @bf << "]"
  end

  def add!(dst, src)
    case src
    when Integer
      @bf.comment "#{dst} += #{src}" if @verbose
      _addsub_const ?+, dst, src
    when String
      @bf.comment "#{dst} += #{src.inspect}.ord" if @verbose
      _addsub_const ?+, dst, src.ord
    when Ptr
      @bf.comment "#{dst} += move(#{src})" if @verbose
      _add(dst, src)
    end
  end

  def add(dst, src, tmp = nil)
    if src.is_a? Ptr
      return alloc_tmp { |ptr| add(dst, src, ptr) } unless tmp
      @bf.comment "#{dst} += #{src}" if @verbose
      _copy(tmp, src)
      _add(dst, tmp)
    else
      add!(dst, src)
    end
  end

  def _sub(dst, src, scale = 1)
    go_to src
    @bf << "[-"
      go_to dst
      @bf << ?- * scale
      go_to src
    @bf << "]"
  end

  def sub!(dst, src)
    case src
    when Integer
      @bf.comment "#{dst} -= #{src}" if @verbose
      _addsub_const ?-, dst, src
    when String
      @bf.comment "#{dst} -= #{src.inspect}.ord" if @verbose
      _addsub_const ?-, dst, src
    when Ptr
      @bf.comment "#{dst} -= move(#{src})" if @verbose
      _sub(dst, src)
    end
  end

  def sub(dst, src, tmp = nil)
    if src.is_a? Ptr
      return alloc_tmp { |ptr| sub(dst, src, ptr) } unless tmp
      @bf.comment "#{dst} -= #{src}" if @verbose
      _copy(tmp, src)
      _sub(dst, tmp)
    else
      add!(dst, src)
    end
  end

  def _times(src)
    go_to src
    @bf << "["
      @bf.indent += 1
      @bf.newline
      yield
      @bf.indent -= 1
      @bf.newline
      go_to src
    @bf << "-]"
  end

  def times!(src, &block)
    @bf.comment "for #{src} = #{src} downto 1:"
    _times(src, &block)
  end

  def times(src, tmp = nil, &block)
    return alloc_tmp { |ptr| times(src, ptr, &block) } unless tmp
    @bf.comment "for #{tmp} = #{src} downto 1:"
    copy tmp, src
    _times(tmp, &block)
  end

  def putchar(src)
    @bf.comment "putchar #{src}" if @verbose
    go_to src
    @bf.putchar
  end

  def putdigit(src)
    @bf.comment "putdigit #{src}" if @verbose
    go_to src
    @bf.putdigit
  end

  def getchar(dst)
    @bf.comment "#{dst} = getchar" if @verbose
    go_to dst
    @bf.getchar
  end

  def getdigit(dst)
    @bf.comment "#{dst} = getdigit" if @verbose
    go_to dst
    @bf.getdigit
  end

  def _mul(dst, src)
    alloc_tmps(2) do |x, y|
      _move x, dst
      _times(src) do
        _copy y, x
        _add dst, y
      end
      _zero x
    end
  end

  def mul!(dst, src)
    case src
    when Integer
      @bf.comment "#{dst} *= #{src}" if @verbose
      alloc_tmp do |tmp|
        _move tmp, dst
        _add dst, tmp, src
      end
    when Ptr
      @bf.comment "#{dst} *= move(#{src})" if @verbose
      _mul(dst, src)
    end
  end

  def mul(dst, src)
    if src.is_a? Ptr
      @bf.comment "#{dst} *= #{src}" if @verbose
      alloc_tmp do |tmp|
        _copy(tmp, src)
        _mul(dst, tmp)
      end
    else
      mul!(dst, src)
    end
  end

  def if_nonzero(src)
    @bf.comment "if #{src} != 0:" if @verbose
    alloc_tmp do |tmp|
      _copy tmp, src  
      go_to tmp
      @bf << "["
        @bf.indent += 1
        @bf.newline
        yield
        @bf.indent -= 1
        @bf.newline
        go_to tmp
        @bf << "[-]"
      @bf << "]"
    end
  end
end
