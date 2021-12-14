class BinaryHeap
  def initialize(heap = [])
    heap.unshift(nil)
    @heap = heap
    (1 .. size / 2).reverse_each do |i|
      downheap(i)
    end
  end

  def [](i)
    @heap[1 + i % size]
  end

  def size
    @heap.size - 1
  end

  def empty?
    size <= 0
  end

  def push(x)
    @heap << x
    upheap(size)
  end

  alias << push

  def pop
    return nil if empty?
    return @heap.pop if size == 1
    x, @heap[1] = @heap[1], @heap.pop
    downheap(1)
    x
  end

  alias shift pop

  private

  def downheap(i)
    while (j = i << 1) <= size
      k = j | 1
      min = i
      min = j if not @heap[min] <= @heap[j]
      min = k if k <= size and not @heap[min] <= @heap[k]
      break if min == i
      @heap[i], @heap[min] = @heap[min], @heap[i]
      i = min
    end
  end

  def upheap(i)
    while i >= 2
      j = i >> 1
      break if @heap[j] <= @heap[i]
      @heap[i], @heap[j] = @heap[j], @heap[i]
      i = j
    end
  end
end
