# Verify してないです

class LCTNode
  # Link Cut Tree では、木をパスに分解し、パスをスプレー木で表す。
  # Link Cut Tree で扱う木を real tree と呼び、パスを表す木を auxiliary tree と呼ぶことにする。

  attr_accessor :parent, :dir, :children, :value

  # @parent: real tree または auxiliary tree 上での親を指す。 real tree 上での根なら nil 。
  # @dir: 親から見た自身の方向を返す。 auxiliary tree 上での根なら nil 。
  # @children: auxiliary tree 上での子。

  def initialize(value)
    @parent = nil
    @dir = nil
    @children = [nil, nil]
    @value = value
  end

  # スプレー木の根か判定する
  def root?
    @dir.nil?
  end

  # 自身を回転して、親と入れ替える
  def rotate
    p, d = @parent, @dir
    @parent = p.parent
    @parent.children[@dir] = self if (@dir = p.dir)
    c, @children[p.dir = 1 ^ d] = @children[1 ^ d], p
    c.dir = d if (p.children[d] = c)
  end

  # スプレー操作
  def splay
    until root?
      if @parent.root?
        rotate
      elsif @dir == @parent.dir
        @parent.rotate
        rotate
      else
        rotate
        rotate
      end
    end
  end

  # real tree の根か判定する
  def real_root?
    @parent.nil?
  end

  # 自身を real tree の根が存在する auxiliary tree に接続する
  # O(log N) amortized
  def expose
    splay
    node = self
    until node.real_root?
      node.parent.children[node.dir = 1] = node
      node = node.parent
      node.splay
    end
    splay
  end

  # 親からの辺を切断して、自身を根とする木ができるようにする
  def cut
    expose
    parent, @children[0] = children[0], nil
    parent.parent = parent.dir = nil
  end

  # new_parent の下に自身を接続する
  def link(new_parent)
    expose
    new_parent.expose
    @parent = new_parent
    new_parent.children[@dir = 1] = self
  end
end
