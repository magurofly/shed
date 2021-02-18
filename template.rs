#![allow(dead_code, unused_imports, unused_macros)]

#[fastout]
fn main() {
  input! {
    n: usize, m: usize,
    s: Chars,
  }
  
  let indices = s.into_iter().enumerate()
    .map(|(i, c)| if c == '0' { i } else { INF })
    .rev()
    .cumprod(INF, cmp::min)
    .to_vec_reversed();

  // eprintln!("{:?}", &indices);

  let mut index = n;
  let mut ans = vec![];
  while index > 0 {
    let next = if index < m { indices[0] } else { indices[index - m] };
    if next >= index {
      println!("-1");
      return;
    }
    ans.push(index - next);
    index = next;
  }

  println!("{}", ans.iter().rev().join(" "));
}

type Int = usize;
const MOD: Int = 1_000_000_007;
const INF: Int = 1_000_000_000;
const YESNO: [&'static str; 2] = ["Yes", "No"];

use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use std::*;
use collections::*; // (BTree|Hash)(Set|Map), BinaryHeap, VecDeque, LinkedList
use cmp::{self, Reverse}; // cmp::{min, max}
use itertools::Itertools as _;
use num_traits::*;
use petgraph::prelude::*; // (Stable)?(Di|Un)?Graph, (Di|Un)?GraphMap, (Node|Edge)Index, Bfs, Dfs, DfsPostOrder
use petgraph::unionfind::UnionFind;

fn yes() { println!("{}", YESNO[0]); }
fn no() { println!("{}", YESNO[1]); }
fn yesno(c: bool) { println!("{}", if c { YESNO[0] } else { YESNO[1] }); }
macro_rules! min {
  ($a:expr $(,)*) => {{ $a }};
  ($a:expr, $b:expr $(,)*) => {{ cmp::min($a, $b) }};
  ($a:expr, $($rest:expr),+ $(,)*) => {{ cmp::min($a, min!($($rest),+)) }}
}
macro_rules! max {
  ($a:expr $(,)*) => {{ $a }};
  ($a:expr, $b:expr $(,)*) => {{ cmp::max($a, $b) }};
  ($a:expr, $($rest:expr),+ $(,)*) => {{ cmp::max($a, max!($($rest),+)) }}
}

trait MyItertools : Iterator {
  fn to_vec(self) -> Vec<Self::Item> where Self: Sized { self.collect::<Vec<_>>() }
  fn to_vec_reversed(self) -> Vec<Self::Item> where Self: Sized { let mut v = self.collect::<Vec<_>>(); v.reverse(); v }
  fn tally(self) -> HashMap<Self::Item, usize> where Self: Sized, Self::Item: Copy + Eq + hash::Hash {
    let mut counts = HashMap::new();
    self.for_each(|item| *counts.entry(item).or_default() += 1 );
    counts
  }
  fn cumprod<F: Fn(Self::Item, Self::Item) -> Self::Item>(self, init: Self::Item, f: F) -> CumProd<Self, Self::Item, F> where Self: Sized, Self::Item: Copy { CumProd { prod: init, f, iter: self } }
}
impl<T: ?Sized> MyItertools for T where T: Iterator {}

struct CumProd<I, P, F> { prod: P, iter: I, f: F }
impl<I: Iterator<Item = P>, P: Copy, F: Fn(P, P) -> P> Iterator for CumProd<I, P, F> {
  type Item = P;
  fn next(&mut self) -> Option<Self::Item> { self.iter.next().map(|x| { self.prod = (self.f)(self.prod.clone(), x); self.prod.clone() }) }
  fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}
impl<I: Iterator<Item = P>, P: Copy, F: Fn(P, P) -> P> ExactSizeIterator for CumProd<I, P, F> {}

trait MyOrd : PartialOrd {
  fn chmax(&mut self, mut rhs: Self) -> &mut Self where Self: Sized { if self < &mut rhs { *self = rhs; }; self }
  fn chmin(&mut self, mut rhs: Self) -> &mut Self where Self: Sized { if self > &mut rhs { *self = rhs; }; self }
}
impl<T: ?Sized> MyOrd for T where T: PartialOrd {}
