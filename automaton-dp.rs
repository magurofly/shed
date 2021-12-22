/// 桁 DP の例
/// この例では、 `0` 以上 `7105` 以下の整数全ての和を計算している
fn main() {
  let s = &[7, 1, 0, 5];
  let n = s.len();

  #[derive(Clone, Copy, PartialEq, Eq, Hash)]
  /// 状態
  enum State {
    Free(usize),
    BoundedU(usize),
  }
  use State::*;

  let mut automaton = AutomatonDP::new(BoundedU(0));
  automaton.accept(BoundedU(n));
  automaton.accept(Free(n));
  for i in 0 .. n {
    for c in 0 ..= 9 {
      automaton.add_transition(Free(i), c, Free(i + 1));
    }
    for c in 0 .. s[i] {
      automaton.add_transition(BoundedU(i), c, Free(i + 1));
    }
    automaton.add_transition(BoundedU(i), s[i], BoundedU(i + 1));
  }

  let ans = automaton.compute(n, |&(x, n), &(y, m)| (x + y, n + m), || (0, 0), |d, &(x, n)| (x * 10 + d * n, n), || (0, 1));
  println!("{}", ans.0);
}



use std::collections::*;
use std::hash::Hash;


/// オートマトン（DFA）が受理するすべての文字列に対して DP をする
/// - `Q`: 状態の型
/// - `C`: 入力の型
pub struct AutomatonDP<Q, C> {
  transition: HashMap<Q, Vec<(Q, C)>>,
  init: Q,
  accept: Vec<Q>,
}
impl<Q, C> AutomatonDP<Q, C>
where
  Q: Copy + Eq + Hash,
  C: Copy + Eq + Hash
{
  /// 初期状態が `init` のオートマトンを作成する
  pub fn new(init: Q) -> Self {
    Self {
      transition: HashMap::new(),
      init,
      accept: Vec::new(),
    }
  }

  /// 遷移を追加する
  /// `from` 状態のとき、 `input` が来ると `to` 状態に遷移する
  pub fn add_transition(&mut self, from: Q, input: C, to: Q) {
    self.transition.entry(from).or_insert_with(Vec::new).push((to, input));
  }

  /// 受理状態を追加する
  pub fn accept(&mut self, state: Q) {
    self.accept.push(state);
  }

  /// 長さ `n` のすべての文字列に対して DP をする
  /// - `op`: 加法
  /// - `e`: 加法単位元を返す
  /// - `map`: 乗法っぽいやつ
  /// - `empty`: 空列に対する結果を返す
  ///
  /// ただし、乗せる値は半環（っぽいやつ）でないといけない
  /// つまり:
  /// ```
  /// assert_eq!((op)(x, (op)(y, z)), (op)((op)(x, y), z));
  /// assert_eq!((op)(x, (e)()), x);
  /// assert_eq!((op)((e)(), x), x);
  /// assert_eq!((op)(x, y), (op)(y, x));
  /// assert_eq!((map)(c, (op)(x, y)), (op)((map)(c, x), (map)(c, y)));
  /// ```
  pub fn compute<S, Op, E, Map, Empty>(&self, n: usize, op: Op, e: E, map: Map, empty: Empty) -> S
  where
    S: Clone + Sized,
    Op: Fn(&S, &S) -> S,
    E: Fn() -> S,
    Map: Fn(C, &S) -> S,
    Empty: Fn() -> S,
  {
    let mut dp = HashMap::new();
    dp.insert(self.init, (empty)());
    for _ in 0 .. n {
      let mut dp2 = HashMap::new();
      for (&from, value) in &dp {
        for &(to, input) in &self.transition[&from] {
          let x = dp2.entry(to).or_insert_with(|| (e)());
          let y = (op)(&x, &(map)(input, value));
          *x = y;
        }
      }
      dp = dp2;
    }
    let mut ans = (e)();
    for &q in &self.accept {
      ans = (op)(&ans, &dp.get(&q).cloned().unwrap_or_else(|| (e)()));
    }
    ans
  }
}
