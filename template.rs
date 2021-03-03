#![allow(dead_code, unused_imports, unused_macros, non_snake_case)]

#[fastout]
fn main() {
  input! {
  }
  
}

type Int = i64;
const MOD: Int = 1_000_000_007;
const INF: Int = 1_000_000_000;
const YESNO: [&'static str; 2] = ["Yes", "No"];

use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use std::*;
use std::ops::*;
use collections::*; // (BTree|Hash)(Set|Map), BinaryHeap, VecDeque, LinkedList
use cmp::{self, Reverse}; // cmp::{min, max}
use itertools::Itertools as _;
use num_traits::*;
use num_integer::*;
use petgraph::prelude::*; // (Stable)?(Di|Un)?Graph, (Di|Un)?GraphMap, (Node|Edge)Index, Bfs, Dfs, DfsPostOrder
use petgraph::unionfind::UnionFind;

fn yes() { println!("{}", YESNO[0]); }
fn no() { println!("{}", YESNO[1]); }
fn yesno(c: bool) { println!("{}", if c { YESNO[0] } else { YESNO[1] }); }
fn neighbor4<F: Fn(usize, usize)>(i: usize, j: usize, h: usize, w: usize, f: F) { if i > 0 { (f)(i - 1, j); } if i < h - 1 { (f)(i + 1, j); } if j > 0 { (f)(i, j - 1); } if j < w - 1 { (f)(i, j + 1); } }

#[macro_export]
macro_rules! min {
  ($a:expr $(,)*) => {{ $a }};
  ($a:expr, $b:expr $(,)*) => {{ cmp::min($a, $b) }};
  ($a:expr, $($rest:expr),+ $(,)*) => {{ cmp::min($a, min!($($rest),+)) }}
}
#[macro_export]
macro_rules! max {
  ($a:expr $(,)*) => {{ $a }};
  ($a:expr, $b:expr $(,)*) => {{ cmp::max($a, $b) }};
  ($a:expr, $($rest:expr),+ $(,)*) => {{ cmp::max($a, max!($($rest),+)) }}
}

trait MyItertools : Iterator {
  fn to_vec(self) -> Vec<Self::Item> where Self: Sized { self.collect::<Vec<_>>() }
  fn to_vec_reversed(self) -> Vec<Self::Item> where Self: Sized { let mut v = self.collect::<Vec<_>>(); v.reverse(); v }
  fn tally(self) -> HashMap<Self::Item, usize> where Self: Sized, Self::Item: Copy + Eq + hash::Hash { let mut counts = HashMap::new(); self.for_each(|item| *counts.entry(item).or_default() += 1 ); counts }
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

trait MyOrd : PartialOrd + Sized {
  fn max(self, other: Self) -> Self { if &self < &other { other } else { self } }
  fn min(self, other: Self) -> Self { if &self > &other { other } else { self } }
  fn chmax(&mut self, mut rhs: Self) -> &mut Self { if self < &mut rhs { *self = rhs; }; self }
  fn chmin(&mut self, mut rhs: Self) -> &mut Self { if self > &mut rhs { *self = rhs; }; self }
}
impl<T: Sized + PartialOrd> MyOrd for T {}

trait MyPrimInt : PrimInt {
  fn mod_pow(self, mut e: Self, m: Self) -> Self { let (mut a, mut r) = (self, Self::one()); while e > Self::zero() { if e & Self::one() == Self::one() { r = r * a % m; } a = a * a % m; e = e >> 1; } r }
  fn ceiling_div(self, other: Self) -> Self { (self + other - Self::one()) / other }
  fn align_floor(self, unit: Self) -> Self { (self / unit) * unit }
  fn align_ceil(self, unit: Self) -> Self { self.ceiling_div(unit) * unit }
}
impl<T: PrimInt> MyPrimInt for T {}

#[derive(Clone, Copy)]
struct ModInt<C>(C, C);
impl<C: PrimInt> ModInt<C> {
  fn new(value: C, modulo: C) -> Self { Self(value, modulo) }
  fn to_int(&self) -> C { (self.0 + self.1) % self.1 }
  fn pow<E: PrimInt>(self, mut exp: E) -> Self { let mut r = C::one(); let mut b = self.0; while !exp.is_zero() { if (exp & E::one()).is_one() { r = r * b % self.1; } b = b * b % self.1; exp = exp >> 1; } Self(r, self.1) }
  fn inv(self) -> Self { self.pow(self.1 - C::one() - C::one()) }
}
impl<C: PrimInt + From<Int>> From<C> for ModInt<C> { fn from(value: C) -> Self { Self(value, From::from(MOD)) } }
impl<C: PrimInt + fmt::Debug> fmt::Debug for ModInt<C> { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({:?} mod {:?})", self.0, self.1) } }
impl<C: PrimInt + fmt::Display> fmt::Display for ModInt<C> { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.to_int().fmt(f) } }
impl<C: PrimInt + From<Int>> Num for ModInt<C> { type FromStrRadixErr = C::FromStrRadixErr; fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> { Ok(Self::from(C::from_str_radix(str, radix)?)) } }
impl<C: PrimInt + From<Int>> PartialEq for ModInt<C> { fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) } }
impl<C: PrimInt + From<Int>> Zero for ModInt<C> { fn zero() -> Self { Self::from(C::zero()) } fn is_zero(&self) -> bool { self.0.is_zero() } }
impl<C: PrimInt + From<Int>> One for ModInt<C> { fn one() -> Self { Self::from(C::one()) } }
impl<C: PrimInt + From<Int>> Add for ModInt<C> { type Output = Self; fn add(self, rhs: Self) -> Self { Self((self.0 + rhs.0) % self.1, self.1) } }
impl<C: PrimInt + From<Int>> Sub for ModInt<C> { type Output = Self; fn sub(self, rhs: Self) -> Self { Self((self.0 - rhs.0) % self.1, self.1) } }
impl<C: PrimInt + From<Int>> Mul for ModInt<C> { type Output = Self; fn mul(self, rhs: Self) -> Self { Self(self.0 * rhs.0 % self.1, self.1) } }
impl<C: PrimInt + From<Int>> Div for ModInt<C> { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
impl<C: PrimInt + From<Int>> Rem for ModInt<C> { type Output = Self; fn rem(self, rhs: Self) -> Self { Self(self.0 % rhs.0, self.1) } }
