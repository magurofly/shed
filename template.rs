#![allow(dead_code, unused_imports, unused_macros, non_snake_case)]

#[fastout]
fn main() {
  input! {
  }
  
  println!("{}", ans);
}

type Int = i64;
const MOD: Int = 1_000_000_007; type Mod = Mod1000000007;
// const MOD: Int = 998244353; type Mod = Mod998244353;
const INF: Int = 1_000_000_000;
const YESNO: [&'static str; 2] = ["Yes", "No"];

use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use std::*;
use std::ops::*;
use collections::*; // (BTree|Hash)(Set|Map), BinaryHeap, VecDeque, LinkedList
use cmp::{self, Reverse}; // cmp::{min, max}
use itertools::*;
use num_traits::*;
use num_integer::*;
use petgraph::prelude::*; // (Stable)?(Di|Un)?Graph, (Di|Un)?GraphMap, (Node|Edge)Index, Bfs, Dfs, DfsPostOrder
use petgraph::unionfind::UnionFind;

type Mint = ModInt<Mod>; fn mint(x: impl ToPrimitive) -> Mint { Mint::new(x.to_i64().unwrap()) }
fn yes() { println!("{}", YESNO[0]); }
fn no() { println!("{}", YESNO[1]); }
fn yesno(c: bool) { println!("{}", if c { YESNO[0] } else { YESNO[1] }); }
fn neighbor4<F: FnMut(usize, usize)>(i: usize, j: usize, h: usize, w: usize, mut f: F) { if i > 0 { (f)(i - 1, j); } if i < h - 1 { (f)(i + 1, j); } if j > 0 { (f)(i, j - 1); } if j < w - 1 { (f)(i, j + 1); } }#[macro_export]
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

pub trait Modulus { fn modulus() -> i64; }
pub struct Mod1000000007; impl Modulus for Mod1000000007 { fn modulus() -> i64 { 1000000007 } }
pub struct Mod998244353; impl Modulus for Mod998244353 { fn modulus() -> i64 { 998244353 } }

pub struct ModInt<M>(i64, std::marker::PhantomData<M>);
impl<M> ModInt<M> {
  pub fn new(n: i64) -> Self where M: Modulus { Self::raw(n.rem_euclid(M::modulus())) }
  pub fn raw(n: i64) -> Self { Self(n, std::marker::PhantomData) }
  pub fn val(&self) -> i64 { self.0 }
}
impl<M> Clone for ModInt<M> { fn clone(&self) -> Self { Self::raw(self.0) } }
impl<M> Copy for ModInt<M> {}
impl<M> PartialEq for ModInt<M> { fn eq(&self, rhs: &Self) -> bool { self.0 == rhs.0 } }
impl<M> Eq for ModInt<M> {}
impl<M> fmt::Debug for ModInt<M> { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(f) } }
impl<M> fmt::Display for ModInt<M> { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.0.fmt(f) } }
impl<M: Modulus> str::FromStr for ModInt<M> { type Err = <i64 as str::FromStr>::Err; fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self::new(i64::from_str(s)?)) } }
impl<M> ToPrimitive for ModInt<M> { fn to_i64(&self) -> Option<i64> { Some(self.0) } fn to_u64(&self) -> Option<u64> { Some(self.0 as u64) } }
impl<M: Modulus> Zero for ModInt<M> { fn zero() -> Self { Self::new(0) } fn is_zero(&self) -> bool { self.0 == 0 } }
impl<M: Modulus> One for ModInt<M> { fn one() -> Self { Self::new(1) } fn is_one(&self) -> bool { self.0 == 1 } }
macro_rules! impl_op { ($M:ident, $T:ident $f:ident, $U:ident $g:ident, |$x:ident, $y:ident| $b:block) => {
  impl <$M: Modulus, A: ToPrimitive> $T<A> for ModInt<$M> { type Output = Self; fn $f(self, y: A) -> Self { let ($x, $y) = (self.0, y.to_i64().unwrap()); $b } }
  impl <$M: Modulus, A: ToPrimitive> $U<A> for ModInt<$M> { fn $g(&mut self, y: A) { *self = self.$f(y); } }
} }
impl_op!(M, Add add, AddAssign add_assign, |x, y| { Self::new(x + y) });
impl_op!(M, Sub sub, SubAssign sub_assign, |x, y| { Self::new(x - y) });
impl_op!(M, Mul mul, MulAssign mul_assign, |x, y| { Self::raw((x as i128 * y as i128).rem_euclid(M::modulus() as i128) as i64) });
impl_op!(M, Div div, DivAssign div_assign, |x, y| { Self::raw(x) * Self::new(y).inv() });
impl_op!(M, Rem rem, RemAssign rem_assign, |x, y| { Self::new(x % y) });
impl<M: Modulus> Num for ModInt<M> { type FromStrRadixErr = <i64 as Num>::FromStrRadixErr; fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> { Ok(Self::new(i64::from_str_radix(str, radix)?)) } }
impl<M: Modulus, A: PrimInt> Pow<A> for ModInt<M> { type Output = Self; fn pow(self, mut e: A) -> Self { let mut r = Self::one(); let mut x = self; while !e.is_zero() { if (e & A::one()).is_one() { r = r * x; } x = x * x; e = e >> 1; } r } }
impl<M: Modulus> Inv for ModInt<M> { type Output = Self; fn inv(self) -> Self { let (mut s, mut t) = ((self.0, 0), (self.0.rem_euclid(M::modulus()), 1)); while t.0 != 0 { let u = s.0 / t.0; s = (s.0 - t.0 * u, s.1 - t.1 * u); std::mem::swap(&mut s, &mut t); } if s.0 < 0 { s.0 = s.0 + M::modulus() / s.0; } Self::raw(s.1) } }
