/// 例: 桁DP
/// l 以上 r 以下の整数の合計
fn main() {
  let digits = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  let lower_bound = &[2, 7, 1, 8, 2, 8, 1, 8, 2, 8];
  let upper_bound = &[3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
  let n = 10;

  let automaton_r = AutomatonDP::digit_lte(digits, upper_bound);
  let automaton_l = AutomatonDP::digit_gte(digits, lower_bound);
  let automaton = automaton_l.intersection(&automaton_r);

  let ans = automaton.compute::<(i64, i64), _, _, _, _>(n, |&(x, n), &(y, m)| (x + y, n + m), || (0, 0), |&(x, n), d| (x * 10 + d * n, n), || (0, 1));
  println!("{}", ans.0);
}


use std::collections::*;
use std::hash::Hash;

/// オートマトン（DFA）が受理するすべての文字列に対して DP をする
/// - `Q`: 状態の型
/// - `C`: 入力の型
pub struct AutomatonDP<Q, C> {
  transition: HashMap<Q, HashMap<C, Q>>,
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
    self.transition.entry(from).or_insert_with(HashMap::new).insert(input, to);
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
    Map: Fn(&S, C) -> S,
    Empty: Fn() -> S,
  {
    let mut dp = HashMap::new();
    dp.insert(self.init, (empty)());
    for _ in 0 .. n {
      let mut dp2 = HashMap::new();
      for (&from, value) in &dp {
        for (&input, &to) in &self.transition[&from] {
          let x = dp2.entry(to).or_insert_with(|| (e)());
          let y = (op)(&x, &(map)(value, input));
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

  /// 論理積をとる
  pub fn intersection<R>(&self, other: &AutomatonDP<R, C>) -> AutomatonDP<(Q, R), C>
  where
    R: Copy + Eq + Hash
  {
    let mut transition = HashMap::new();
    for (&p1, tr1) in &self.transition {
      for (&c, &q1) in tr1 {
        for (&p2, tr2) in &other.transition {
          if let Some(&q2) = tr2.get(&c) {
            transition.entry((p1, p2)).or_insert_with(HashMap::new).insert(c, (q1, q2));
          }
        }
      }
    }
    let mut accept = Vec::new();
    for &q1 in &self.accept {
      for &q2 in &other.accept {
        accept.push((q1, q2));
      }
    }
    AutomatonDP {
      init: (self.init, other.init),
      transition,
      accept,
    }
  }
}

impl<C> AutomatonDP<usize, C>
where
  C: Copy + Eq + Hash
{
  /// 辞書順で A 以下の数列を受理するオートマトンを作成
  /// 頂点数 O(upper_bound.len())
  /// 辺数 O(upper_bound.len() * digits.len())
  pub fn digit_lte(digits: &[C], upper_bound: &[C]) -> Self {
    let n = upper_bound.len();
    let mut automaton = AutomatonDP::new(0);
    automaton.accept(n);
    automaton.accept(2 * n + 1);
    for i in 0 .. n {
      for &c in digits {
        automaton.add_transition(n + 1 + i, c, n + 1 + i + 1);
      }
      for &c in digits {
        if upper_bound[i] == c {
          automaton.add_transition(i, c, i + 1);
          break;
        }
        automaton.add_transition(i, c, n + 1 + i + 1);
      }
    }
    automaton
  }

  /// 辞書順で A 以上の数列を受理するオートマトンを作成
  /// 頂点数 O(upper_bound.len())
  /// 辺数 O(upper_bound.len() * digits.len())
  pub fn digit_gte(digits: &[C], lower_bound: &[C]) -> Self {
    let n = lower_bound.len();
    let mut automaton = AutomatonDP::new(0);
    automaton.accept(n);
    automaton.accept(2 * n + 1);
    for i in 0 .. n {
      for &c in digits {
        automaton.add_transition(n + 1 + i, c, n + 1 + i + 1);
      }
      for &c in digits.into_iter().rev() {
        if lower_bound[i] == c {
          automaton.add_transition(i, c, i + 1);
          break;
        }
        automaton.add_transition(i, c, n + 1 + i + 1);
      }
    }
    automaton
  }
}
