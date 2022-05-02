  #![allow(dead_code, unused_imports, unused_macros, non_snake_case)]

fn main() {
  let mut stdin = io::stdin();

}

type Int = i64;
const MOD: Int = 1_000_000_007;
// const MOD: Int = 998244353;
const INF: Int = 1_000_000_000;
const YESNO: [&'static str; 2] = ["Yes", "No"];

use std::*;
use std::ops::*;
use collections::*; // (BTree|Hash)(Set|Map), BinaryHeap, VecDeque, LinkedList
use cmp::{self, Reverse}; // cmp::{min, max}

fn yes() { println!("{}", YESNO[0]); }
fn no() { println!("{}", YESNO[1]); }
fn yesno(c: bool) { println!("{}", if c { YESNO[0] } else { YESNO[1] }); }
fn neighbor4<F: FnMut(usize, usize)>(i: usize, j: usize, h: usize, w: usize, mut f: F) { if i > 0 { (f)(i - 1, j); } if i < h - 1 { (f)(i + 1, j); } if j > 0 { (f)(i, j - 1); } if j < w - 1 { (f)(i, j + 1); } }
fn read_line(stdin: &mut std::io::Stdin) -> String { let mut str = String::new(); stdin.read_line(&mut str).expect("read_line"); str }
fn read_words(stdin: &mut std::io::Stdin) -> Vec<String> { read_line(stdin).split_ascii_whitespace().map(str::to_string).collect() }
fn read_parse<T: str::FromStr>(stdin: &mut std::io::Stdin) -> Vec<T> where T::Err: std::fmt::Debug { read_words(stdin).iter().map(|s| s.parse()).map(|r| r.unwrap()).collect() }
fn assign<T>(vars: &mut [&mut T], vals: Vec<T>) { for (var, val) in vars.iter_mut().zip(vals) { **var = val; } }
fn take<T: Default + Copy, const N: usize>(vals: &[T]) -> [T; N] { let mut arr = [T::default(); N]; for (i, &x) in vals.iter().enumerate() { arr[i] = x; }; arr }

trait MyItertools : Iterator {
  fn to_vec(self) -> Vec<Self::Item> where Self: Sized { self.collect::<Vec<_>>() }
  fn to_vec_reversed(self) -> Vec<Self::Item> where Self: Sized { let mut v = self.collect::<Vec<_>>(); v.reverse(); v }
  fn tally(self) -> HashMap<Self::Item, usize> where Self: Sized, Self::Item: Copy + Eq + hash::Hash { let mut counts = HashMap::new(); self.for_each(|item| *counts.entry(item).or_default() += 1 ); counts }
}
impl<T: ?Sized> MyItertools for T where T: Iterator {}

trait MyOrd : PartialOrd + Sized {
  fn chmax(&mut self, mut rhs: Self) -> bool { if self < &mut rhs { *self = rhs; true } else { false } }
  fn chmin(&mut self, mut rhs: Self) -> bool { if self > &mut rhs { *self = rhs; true } else { false } }
}
impl<T: Sized + PartialOrd> MyOrd for T {}
