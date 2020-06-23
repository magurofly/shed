#!title 二分ヒープ
#!description 最小。追加O(1)、削除O(log n)
#!tags データ構造 ヒープ 二分ヒープ 優先度付きキュー
class HeapBinary
	# @implements Heap

	def initialize(&comparator)
		@cmp, @arr = comparator, []
	end

	def <<(x)
		@arr << x
		i = @arr.size
		j = i >> 1
		while j > 1 and @cmp[@arr[i], @arr[j]] < 0
			@arr[i], @arr[j] = @arr[j], @arr[i]
			i, j = j, j >> 1
		end
	end

	def shift
		x, @arr[0] = @arr[0], @arr.pop
		i, j, n = 0, 1, @arr.size
		while j < n
			j += 1 if j+1 < n and @cmp[@arr[j], @arr[j+1]] > 0
			@arr[i], @arr[j] = @arr[j], @arr[i] if @cmp[@arr[i], @arr[j]] > 0
			i, j = j, i << 1 | 1
		end
	end
end
