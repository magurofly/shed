use rand::prelude::*;

/*
参考にしたもの
https://ja.wikipedia.org/wiki/%E3%82%AF%E3%82%A4%E3%83%83%E3%82%AF%E3%82%BB%E3%83%AC%E3%82%AF%E3%83%88
https://naoyat.hatenablog.jp/entry/median-of-medians
*/

fn main() {
    let mut xs = (1 ..= 30).collect::<Vec<_>>();
    xs.shuffle(&mut thread_rng());
    println!("{xs:?}");
    for i in 0 .. xs.len() {
        let mut ys = xs.clone();
        select_kth(&mut ys, i);
        println!("{i}: {}", ys[i]);
    }
}


/// k 番目に小さい値が slice[k] になるよう並び替える
/// 時間計算量: O(slice.len())
/// 空間計算量: O(log(slice.len()))
fn select_kth<T: Ord + std::fmt::Debug>(slice: &mut [T], k: usize) {
    /// k 番目に小さい値が slice[k] になるよう並び替える
    fn select_kth_inner<T: Ord + std::fmt::Debug>(mut slice: &mut [T], mut k: usize) {
        while !slice.is_empty() {
            // 10 個以下ならソートする（定数なのでゆるして）
            if slice.len() <= 5 {
                slice.sort();
                return;
            }
            // println!("slice = {:?}", slice);
            let (n_le_pivot, pivot_index) = partition_pivot(slice);
            // println!("partitioned by {:?}: {:?} and {:?}", &slice[pivot_index], &slice[.. n_le_pivot], &slice[n_le_pivot ..]);
            if k + 1 == n_le_pivot {
                // k 番目がちょうど pivot の場合
                slice.swap(k, pivot_index);
                return;
            } else if k < n_le_pivot {
                // k 番目の値が pivot 以下の場合
                slice.swap(n_le_pivot - 1, pivot_index);
                slice = &mut slice[.. n_le_pivot - 1];
            } else {
                // k 番目の値が pivot よりも大きい場合
                slice = &mut slice[n_le_pivot ..];
                k -= n_le_pivot;
            }
        }
    }

    // 中央値くらいの要素を pivot とし、 pivot 以下の要素と pivot より大きい要素に分ける
    // (境界の位置, pivot より小さい値の個数, pivot と等しい値の個数) を返す
    fn partition_pivot<T: Ord + std::fmt::Debug>(slice: &mut [T]) -> (usize, usize) {
        // 5 個ずつブロックに分けて各ブロックの中央値を求め、その後各ブロックの中央値をスライスの先頭に移動する
        let n_medians = (slice.len() + 4) / 5;
        for i in 0 .. n_medians {
            let l = i * 5;
            let r = slice.len().min(i * 5 + 5);
            slice[l .. r].sort();
            slice.swap(i, (l + r) / 2);
        }

        // slice[n_medians / 2] が pivot になるようにする
        let mut pivot_index = n_medians / 2;
        select_kth(&mut slice[0 .. n_medians], pivot_index);
        // pivot 以下の値と pivot より大きい値に分割する
        let mut l = 0;
        let mut r = slice.len();
        while l < r {
            if slice[l] <= slice[pivot_index] {
                l += 1
            } else {
                if slice[r - 1] == slice[pivot_index] {
                    pivot_index = l;
                }
                slice.swap(l, r - 1);
                r -= 1;
            }
        }
        (l, pivot_index)
    }

    if slice.len() <= 1 {
      return;
    }
    select_kth_inner(slice, k)
}
