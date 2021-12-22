fn main() {
  let s = &[7, 1, 0, 5];
  let n = s.len();

  #[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
  pub fn new(init: Q) -> Self {
    Self {
      transition: HashMap::new(),
      init,
      accept: Vec::new(),
    }
  }

  pub fn add_transition(&mut self, from: Q, input: C, to: Q) {
    self.transition.entry(from).or_insert_with(Vec::new).push((to, input));
  }

  pub fn accept(&mut self, state: Q) {
    self.accept.push(state);
  }

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
