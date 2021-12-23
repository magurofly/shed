/// 例: 桁DP
/// ## 問題文
/// 正整数 L, R, M が与えられます。 L, R は共に 10 進法表現の長さが N の整数です。
/// L, R をそれぞれ M/N 回繰り返したものを L', R' とします。
/// L' 以上 R' 以下の整数の総和を求めてください。
/// ## 制約
/// - $1 \le L \le R \le 10^{1000}
/// - $1 \le M \le 1000$
fn main() {
  let digits = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  let lower_bound = &[1, 2, 3];
  let upper_bound = &[3, 2, 1];
  let n = 3;
  let m = 7;

  // let automaton_r = AutomatonDP::digit_lte(digits, upper_bound);
  // let automaton_l = AutomatonDP::digit_gte(digits, lower_bound);
  // let automaton = automaton_l.intersection(&automaton_r);
  let mut automaton = AutomatonDP::digit_between(digits, lower_bound, upper_bound);
  automaton.reject_all();
  automaton.accept(DigitDPState::BothBounded(m % n));
  automaton.accept(DigitDPState::UpperBounded(m % n));
  automaton.accept(DigitDPState::LowerBounded(m % n));
  automaton.accept(DigitDPState::Unbounded(m % n));

  let ans = automaton.compute::<(i64, i64), _, _, _, _>(m, |&(x, n), &(y, m)| (x + y, n + m), || (0, 0), |&(x, n), d| (x * 10 + d * n, n), || (0, 1));
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

  /// 遷移を削除する
  pub fn remove_transition(&mut self, from: Q, input: C) {
    self.transition.entry(from).and_modify(|tr| {
      tr.remove(&input);
    });
  }

  /// 受理状態を追加する
  pub fn accept(&mut self, state: Q) {
    self.accept.push(state);
  }

  /// 受理状態を削除する
  /// O(accept.len())
  pub fn reject(&mut self, state: Q) {
    let mut prev = 0;
    while let Some(index) = (prev .. self.accept.len()).find(|&i| self.accept[i] == state) {
      self.accept.swap_remove(index);
      prev = index;
    }
  }

  /// 受理状態をクリアする
  pub fn reject_all(&mut self) {
    self.accept.clear();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DigitDPState {
  BothBounded(usize),
  LowerBounded(usize),
  UpperBounded(usize),
  Unbounded(usize),
}
impl<C> AutomatonDP<DigitDPState, C>
where
  C: Copy + Eq + Hash
{
  /// 辞書順で upper_bound 以下の数列を受理するオートマトンを作成
  /// 頂点数 O(upper_bound.len())
  /// 辺数 O(upper_bound.len() * digits.len())
  pub fn digit_lte(digits: &[C], upper_bound: &[C]) -> Self {
    use DigitDPState::*;
    let n = upper_bound.len();
    let mut automaton = AutomatonDP::new(UpperBounded(0));
    automaton.accept(UpperBounded(n));
    automaton.accept(Unbounded(n));
    for i in 0 ..= n {
      let r = upper_bound[i % n];
      for &c in digits {
        automaton.add_transition(Unbounded(i), c, Unbounded(i % n + 1));
      }
      for &c in digits {
        if c == r {
          automaton.add_transition(UpperBounded(i), c, UpperBounded(i % n + 1));
          break;
        }
        automaton.add_transition(UpperBounded(i), c, Unbounded(i % n + 1));
      }
    }
    automaton
  }

  /// 辞書順で lower_bound 以上の数列を受理するオートマトンを作成
  /// 頂点数 O(lower_bound.len())
  /// 辺数 O(lower_bound.len() * digits.len())
  pub fn digit_gte(digits: &[C], lower_bound: &[C]) -> Self {
    use DigitDPState::*;
    let n = lower_bound.len();
    let mut automaton = AutomatonDP::new(LowerBounded(0));
    automaton.accept(LowerBounded(n));
    automaton.accept(Unbounded(n));
    for i in 0 ..= n {
      let l = lower_bound[i % n];
      for &c in digits {
        automaton.add_transition(Unbounded(i), c, Unbounded(i % n + 1));
      }
      for &c in digits.into_iter().rev() {
        if c == l {
          automaton.add_transition(LowerBounded(i), c, LowerBounded(i % n + 1));
          break;
        }
        automaton.add_transition(LowerBounded(i), c, Unbounded(i % n + 1));
      }
    }
    automaton
  }

  /// 辞書順で lower_bound 以上 upper_bound 以下の数列を受理するオートマトンを作成
  pub fn digit_between(digits: &[C], lower_bound: &[C], upper_bound: &[C]) -> Self {
    use DigitDPState::*;
    assert_eq!(upper_bound.len(), lower_bound.len());
    let n = lower_bound.len();
    let s = digits.len();
    let mut automaton = AutomatonDP::new(BothBounded(0));
    automaton.accept(BothBounded(n));
    automaton.accept(LowerBounded(n));
    automaton.accept(UpperBounded(n));
    automaton.accept(Unbounded(n));
    for i in 0 ..= n {
      let (l, r) = (lower_bound[i % n], upper_bound[i % n]);
      for &c in digits {
        automaton.add_transition(Unbounded(i), c, Unbounded(i % n + 1));
      }
      for &c in digits.into_iter().rev() {
        if c == l {
          automaton.add_transition(LowerBounded(i), c, LowerBounded(i % n + 1));
          break;
        }
        automaton.add_transition(LowerBounded(i), c, Unbounded(i % n + 1));
      }
      for &c in digits {
        if c == r {
          automaton.add_transition(UpperBounded(i), c, UpperBounded(i % n + 1));
          break;
        }
        automaton.add_transition(UpperBounded(i), c, Unbounded(i % n + 1));
      }
      let lower = (0 .. s).find(|&j| digits[j] == l).unwrap();
      let upper = (0 .. s).find(|&j| digits[j] == r).unwrap();
      if lower == upper {
        automaton.add_transition(BothBounded(i), digits[lower], BothBounded(i % n + 1));
      } else if lower < upper {
        automaton.add_transition(BothBounded(i), digits[lower], LowerBounded(i % n + 1));
        automaton.add_transition(BothBounded(i), digits[upper], UpperBounded(i % n + 1));
        for &c in &digits[lower + 1 .. upper] {
          automaton.add_transition(BothBounded(i), c, Unbounded(i % n + 1));
        }
      }
    }
    automaton
  }
}
