#![allow(dead_code, unused_imports, unused_macros, non_snake_case)]

fn main() {
  input! {
    
  }
  
}

type Int = i64;
const MOD: Int = 998244353;
// const MOD: Int = 1_000_000_007;
const INF: Int = 1_000_000_000_000_000_000;
const YESNO: [&'static str; 2] = ["Yes", "No"];

use proconio::{input, input_interactive, marker::{Chars, Bytes, Usize1}};
use std::*;
use std::ops::*;
use collections::*; // (BTree|Hash)(Set|Map), BinaryHeap, VecDeque, LinkedList
use cmp::{self, Reverse}; // cmp::{min, max}
use itertools::*;
use num_traits::*;
use num_integer::*;
use permutohedron::*;
use ac_library::*; type Mint = ModInt998244353;

fn yes() { println!("{}", YESNO[0]); }
fn no() { println!("{}", YESNO[1]); }
fn yesno(c: bool) { println!("{}", if c { YESNO[0] } else { YESNO[1] }); }
fn say<T: std::fmt::Display>(x: T) -> T { println!("{}", x); x }
fn neighbor4<F: FnMut(usize, usize)>(i: usize, j: usize, h: usize, w: usize, mut f: F) { if i > 0 { (f)(i - 1, j); } if i < h - 1 { (f)(i + 1, j); } if j > 0 { (f)(i, j - 1); } if j < w - 1 { (f)(i, j + 1); } }

trait MyItertools : Iterator + Sized {
  fn to_vec(self) -> Vec<Self::Item> { self.collect::<Vec<_>>() }
  fn to_vec_rev(self) -> Vec<Self::Item> { let mut v = self.collect::<Vec<_>>(); v.reverse(); v }
  fn tally(self) -> HashMap<Self::Item, usize> where Self::Item: Copy + Eq + hash::Hash { let mut counts = HashMap::new(); self.for_each(|item| *counts.entry(item).or_default() += 1 ); counts }
  fn count_if<P: Fn(&Self::Item) -> bool>(self, predicate: P) -> usize { self.filter(predicate).count() }
  fn implode(self, sep: &str) -> String where Self::Item: std::string::ToString { self.map(|x| x.to_string()).to_vec().join(sep) }
  fn mex(self, gen: impl IntoIterator<Item = Self::Item>) -> Self::Item where Self::Item: Ord { let mut v = self.collect::<Vec<_>>(); v.sort(); v.dedup(); let mut it = v.into_iter(); gen.into_iter().find(|a| if let Some(x) = it.next() { a != &x } else { true }).unwrap() }
}
impl<T: ?Sized> MyItertools for T where T: Iterator + Sized {}

trait MyOrd : PartialOrd + Sized {
  fn chmax(&mut self, mut rhs: Self) -> bool { if self < &mut rhs { *self = rhs; true } else { false } }
  fn chmin(&mut self, mut rhs: Self) -> bool { if self > &mut rhs { *self = rhs; true } else { false } }
}
impl<T: Sized + PartialOrd> MyOrd for T {}

trait MyPrimInt : PrimInt {
  /// `self ** e` を `m` で割ったあまりを返す
  fn mod_pow(self, mut e: Self, m: Self) -> Self { let (mut a, mut r) = (self, Self::one()); while e > Self::zero() { if e & Self::one() == Self::one() { r = r * a % m; } a = a * a % m; e = e >> 1; } r }
  /// 互いに素な場合、 `mod m` における `self` の逆元、そうでない場合、 `self * inv % m == gcd(self, m)` となるような `Err(inv)` を返す
  fn mod_inv(self, m: Self) -> Result<Self, Self> { let (g, i) = self.ext_gcd(m); if g.is_one() { Ok(i) } else { Err(i) } }
  /// 素数判定をする。 `self < 2**64` の場合は `O(log self)` 、そうでなければ `O(sqrt self)`
  fn is_prime(self) -> bool { let u = Self::zero().count_zeros() - self.leading_zeros(); if u <= 8 || u > 64 { self._is_prime_sqrt() } else { self._is_prime_miller_rabin(&if u <= 32 { vec![2, 7, 61] } else { vec![2, 3, 5, 7, 11, 13, 17] }.into_iter().map(MyPrimInt::convert).collect::<Vec<_>>()) } }
  fn _is_prime_sqrt(self) -> bool { let mut k = 2.convert(); while k * k <= self { if (self % k).is_zero() { return false; } k = k + Self::one(); } true }
  fn _is_prime_miller_rabin(self, bases: &[Self]) -> bool { if self <= Self::one() { return false; } for &a in bases { if self == a { return true; } if (self % a).is_zero() { return false; } } let mut d = self - Self::one(); d = d >> d.trailing_zeros() as usize; for &a in bases { let mut t = d; let mut y = a.mod_pow(t, self); while !(self - t).is_one() && !y.is_one() && !(self - y).is_one() { y = y * y % self; t = t << 1; } if !(self - y).is_one() && (t % 2.convert()).is_zero() { return false; } } true }
  /// 拡張ユークリッドの互助法 `(gcd(self, a), x)` s.t. `self * x + a * y == gcd(self, a)` for some y
  // fn ext_gcd(self, a: Self) -> (Self, Self, Self) { if self < a { let (g, y, x) = a.ext_gcd(self); (g, x, y) } else if a.is_zero() { (self, Self::one(), Self::one()) } else { let (g, y, x) = a.ext_gcd(self % a); (g, x, y - x * (self / a)) } }
  fn ext_gcd(self, x: Self) -> (Self, Self) { let y = self % x; if y.is_zero() { return (x, y); } let (mut p, mut q) = ((x, Self::zero()), (y, Self::one())); while !q.0.is_zero() { let u = p.0 / q.0; p = (p.0 - q.0 * u, p.1 - q.1 * u); std::mem::swap(&mut p, &mut q); } if p.0 < Self::zero() { p.0 = p.0 + x / p.0; } p }
  /// `(self / other).ceil()`
  fn ceiling_div(self, other: Self) -> Self { (self + other - Self::one()) / other }
  /// `(self / unit).floor() * unit`
  fn align_floor(self, unit: Self) -> Self { (self / unit) * unit }
  /// `(self / unit).ceil() * unit`
  fn align_ceil(self, unit: Self) -> Self { self.ceiling_div(unit) * unit }
  /// 他の整数型に変換する
  fn convert<T: PrimInt>(self) -> T { <T as NumCast>::from(self).unwrap() }
  fn pow(self, mut e: Self) -> Self { let (mut a, mut r) = (self, Self::one()); while e > Self::zero() { if e & Self::one() == Self::one() { r = r * a; } a = a * a; e = e >> 1; } r }
  /// base を基数としたときの各桁を求める（ 0 のときは vec![0] ）
  fn digits(mut self, base: Self) -> Vec<Self> { assert!(base > Self::zero()); let mut d = vec![]; while self != Self::zero() { d.push(self % base); self = self / base; } d.reverse(); if d.is_empty() { d.push(Self::zero()); }; d }
}
impl<T: PrimInt> MyPrimInt for T {}

#[derive(Debug, Clone, Default)]
pub struct BTreeMultiset<T: Ord> { len: usize, set: BTreeMap<T, usize> }
impl<'a, T: Ord> BTreeMultiset<T> {
  pub fn new() -> Self { Self { len: 0, set: BTreeMap::new() } }
  pub fn len(&self) -> usize { self.len }
  pub fn count(&self, x: &T) -> usize { self.set.get(x).copied().unwrap_or(0) }
  pub fn insert_multiple(&mut self, x: T, count: usize) -> usize { self.len += count; let n = self.set.entry(x).or_insert(0); *n += count; *n }
  pub fn insert(&mut self, x: T) -> usize { self.insert_multiple(x, 1) }
  pub fn remove_multiple(&mut self, x: &T, count: usize) -> usize { if let Some(n) = self.set.get_mut(x) { let n0 = *n; *n = n0.saturating_sub(count); let n = *n; self.len -= n0 - n; if n == 0 { self.set.remove(x); } n } else { 0 } }
  pub fn remove(&mut self, x: &T) -> usize { self.remove_multiple(x, 1) }
  pub fn iter(&'a self) -> btree_map::Iter<'a, T, usize> { self.set.iter() }
  pub fn into_iter(self) -> btree_map::IntoIter<T, usize> { self.set.into_iter() }
  pub fn keys(&'a self) -> btree_map::Keys<'a, T, usize> { self.set.keys() }
  pub fn range(&'a self, range: impl RangeBounds<T>) -> btree_map::Range<'a, T, usize> { self.set.range(range) }
}

#[derive(Clone, Debug)]
pub struct RectangleSum { imos: Vec<Vec<i64>>, h: usize, w: usize }
impl RectangleSum {
  pub fn new(data: &[Vec<i64>]) -> Self {
    let h = data.len();
    let w = data[0].len();
    let mut imos = vec![vec![0; w + 1]; h + 1];
    for i in 0 .. h { for j in 0 .. w {
      let sum = imos[i + 1][j] + imos[i][j + 1] - imos[i][j] + data[i][j];
      imos[i + 1][j + 1] = sum;
    } }
    Self { imos, h, w }
  }
  
  pub fn sum(&self, i: impl std::ops::RangeBounds<usize>, j: impl std::ops::RangeBounds<usize>) -> i64 {
    use std::ops::Bound::*;
    let i0 = match i.start_bound() { Included(&i) => i, Excluded(&i) => i + 1, Unbounded => 0 };
    let i1 = match i.end_bound() { Included(&i) => i + 1, Excluded(&i) => i, Unbounded => self.h };
    let j0 = match j.start_bound() { Included(&j) => j, Excluded(&j) => j + 1, Unbounded => 0 };
    let j1 = match j.end_bound() { Included(&j) => j + 1, Excluded(&j) => j, Unbounded => self.w };
    self.imos[i1][j1] - self.imos[i1][j0] + self.imos[i0][j0] - self.imos[i0][j1]
  }
}

pub struct Factorial<M: ModIntBase> {
  fact: Vec<M>,
  fact_inv: Vec<M>,
}
impl<M: ModIntBase> Factorial<M> {
  pub fn new() -> Self { Self { fact: vec![M::from(1)], fact_inv: vec![M::from(1)] } }

  pub fn fact(&mut self, n: usize) -> M { self.ensure(n); self.fact[n] }

  pub fn fact_inv(&mut self, n: usize) -> M { self.ensure(n); self.fact_inv[n] }

  /// 二項係数（組合せ）
  pub fn binom(&mut self, n: usize, r: usize) -> M {
    if r > n { return M::from(0); }
    self.ensure(n);
    self.fact[n] * self.fact_inv[n - r] * self.fact_inv[r]
  }

  /// 順列
  pub fn perm(&mut self, n: usize, r: usize) -> M {
    if r > n { return M::from(0); }
    self.ensure(n);
    self.fact[n] * self.fact_inv[n - r]
  }

  /// 重複組合せ
  pub fn homo(&mut self, n: usize, r: usize) -> M {
    if n + r == 0 { return M::from(1); }
    self.binom(n + r - 1, r)
  }

  pub fn ensure(&mut self, n: usize) {
    if n < self.fact.len() {
      return;
    }
    let old_len = self.fact.len();
    let new_len = (n + 1).next_power_of_two();
    self.fact.resize(new_len, M::from(1));
    self.fact_inv.resize(new_len, M::from(1));
    for i in old_len .. new_len {
      self.fact[i] = self.fact[i - 1] * M::from(i);
    }
    self.fact_inv[new_len - 1] = self.fact[new_len - 1].inv();
    for i in (old_len .. new_len - 1).rev() {
      self.fact_inv[i] = self.fact_inv[i + 1] * M::from(i + 1);
    }
  }
}
