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
  fn count_if<P: Fn(Self::Item) -> bool>(self, predicate: P) -> usize { self.map(predicate).filter(|&x| x ).count() }
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

trait MyRangeBounds<T> : RangeBounds<T> {
  /// 単調性を持つ条件を与えたとき、区間に含まれる点のうち、条件を満たす最小の点を求める。
  /// そのような点が存在しなければ `None` を返す。
  fn lower_bound(&self, mut f: impl FnMut(T) -> bool) -> Option<T> where T: PrimInt + Bounded {
    use std::ops::Bound::*;
    let mut min = match self.start_bound() { Included(&l) => l, Excluded(&l) => l + T::one(), Unbounded => T::min_value() };
    let mut max = match self.end_bound() { Included(&r) => r, Excluded(&r) => r - T::one(), Unbounded => T::max_value() };
    if min > max || !f(max) {
      return None;
    }
    while min < max {
      let mid = min + ((max - min) >> 1);
      if f(mid) {
        max = mid;
      } else {
        min = mid + T::one();
      }
    }
    Some(min)
  }
}
impl<T, R: RangeBounds<T>> MyRangeBounds<T> for R {}

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

pub struct Factorial<M: ac_library::modint::ModIntBase> {
  fact: Vec<M>,
  fact_inv: Vec<M>,
}
impl<M: ac_library::modint::ModIntBase> Factorial<M> {
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

use adjacent_list::{new_unweighted, new_weighted, AdjacentList};
pub mod adjacent_list {
  pub fn new_unweighted(n: usize) -> Vec<Vec<usize>> { vec![vec![]; n] }
  pub fn new_weighted<T>(n: usize) -> Vec<Vec<(usize, T)>> { (0 .. n).map(|_| vec![] ).collect() }
  
  pub trait AdjacentList {
    type T;
    fn n(&self) -> usize;
    fn deg(&self, from: usize) -> usize;
    fn arc(&self, from: usize, index: usize) -> (usize, &Self::T);
    fn adjacents(&self, from: usize) -> Iter<'_, Self> {
      Iter::new(self, from)
    }
    fn add_arc(&mut self, from: usize, to: usize, weight: Self::T);
    fn add_edge(&mut self, u: usize, v: usize, weight: Self::T) where Self::T: Clone {
      self.add_arc(u, v, weight.clone());
      self.add_arc(v, u, weight);
    }
    
    fn add_arcs(&mut self, arcs: impl IntoIterator<Item = impl Arc<Self::T>>) {
      for arc in arcs {
        let (u, v, w) = arc.into_tuple();
        self.add_arc(u, v, w);
      }
    }
    fn add_edges(&mut self, arcs: impl IntoIterator<Item = impl Arc<Self::T>>) where Self::T: Clone {
      for arc in arcs {
        let (u, v, w) = arc.into_tuple();
        self.add_edge(u, v, w);
      }
    }
    
    fn dfs_preorder(&self, start: usize, visited: &mut [bool], mut f: impl FnMut(usize, usize, &Self::T) -> bool) {
      visited[start] = true;
      let mut stack = vec![(start, 0)];
      while let Some((u, i)) = stack.pop() {
        for j in i .. self.deg(u) {
          let (v, w) = self.arc(u, j);
          if !visited[v] && f(u, v, w) {
            visited[v] = true;
            if j + 1 < self.deg(u) {
              stack.push((u, j + 1));
            }
            stack.push((v, 0));
            break;
          }
        }
      }
    }
    
    fn connected_components(&self) -> Vec<Vec<usize>> {
      let mut components = vec![];
      let mut visited = vec![false; self.n()];
      for start in 0 .. self.n() {
        if !visited[start] {
          let mut component = vec![start];
          self.dfs_preorder(start, &mut visited, |_, v, _| {
            component.push(v);
            true
          });
          components.push(component);
        }
      }
      components
    }
    
    /// - `starts`: 始点（複数可）
    /// - `zero`: 距離 $0$ の値
    /// - `inf`: 距離 $∞$ の値
    /// - `next_dist(d_u, u, v, w) = d_w`: 距離関数
    fn dijkstra<D>(&self, starts: &[usize], zero: D, inf: D, mut next_dist: impl FnMut(&D, usize, usize, &Self::T) -> D) -> Vec<D> where D: Clone + Ord {
      let mut dist = vec![inf; self.n()];
      let mut pq = std::collections::BinaryHeap::new();
      for &start in starts {
        dist[start] = zero.clone();
        pq.push((std::cmp::Reverse(zero.clone()), start));
      }
      while let Some((std::cmp::Reverse(d), u)) = pq.pop() {
        if dist[u] < d {
          continue;
        }
        for (v, w) in self.adjacents(u) {
          let d2 = next_dist(&d, u, v, w);
          if dist[v] > d2 {
            dist[v] = d2.clone();
            pq.push((std::cmp::Reverse(d2), v));
          }
        }
      }
      dist
    }
  }
  impl AdjacentList for Vec<Vec<usize>> {
    type T = ();
    fn n(&self) -> usize { self.len() }
    fn deg(&self, from: usize) -> usize { self[from].len() }
    fn arc(&self, from: usize, index: usize) -> (usize, &()) { (self[from][index], &()) }
    fn add_arc(&mut self, from: usize, to: usize, _: ()) {
      assert!(from < self.n() && to < self.n());
      self[from].push(to);
    }
  }
  impl<T> AdjacentList for Vec<Vec<(usize, T)>> {
    type T = T;
    fn n(&self) -> usize {
      self.len()
    }
    fn deg(&self, from: usize) -> usize { self[from].len() }
    fn arc(&self, from: usize, index: usize) -> (usize, &T) {
      let &(v, ref w) = &self[from][index];
      (v, w)
    }
    fn add_arc(&mut self, from: usize, to: usize, weight: T) {
      assert!(from < self.n() && to < self.n());
      self[from].push((to, weight));
    }
  }
  
  pub struct Iter<'a, G: ?Sized> {
    graph: &'a G,
    from: usize,
    index: usize,
  }
  impl<'a, G: AdjacentList + ?Sized> Iter<'a, G> {
    pub fn new(graph: &'a G, from: usize) -> Self {
      Self { graph, from, index: 0 }
    }
  }
  impl<'a, G: AdjacentList + ?Sized> Iterator for Iter<'a, G> {
    type Item = (usize, &'a G::T);
    fn next(&mut self) -> Option<Self::Item> {
      if self.index < self.graph.deg(self.from) {
        let item = self.graph.arc(self.from, self.index);
        self.index += 1;
        Some(item)
      } else {
        None
      }
    }
  }
  
  pub trait Arc<T> {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn weight(&self) -> &T;
    fn into_tuple(self) -> (usize, usize, T);
  }
  impl Arc<()> for (usize, usize) {
    fn from(&self) -> usize { self.0 }
    fn to(&self) -> usize { self.1 }
    fn weight(&self) -> &() { &() }
    fn into_tuple(self) -> (usize, usize, ()) { (self.0, self.1, ()) }
  }
  impl Arc<()> for &(usize, usize) {
    fn from(&self) -> usize { self.0 }
    fn to(&self) -> usize { self.1 }
    fn weight(&self) -> &() { &() }
    fn into_tuple(self) -> (usize, usize, ()) { (self.0, self.1, ()) }
  }
  impl<T> Arc<T> for (usize, usize, T) {
    fn from(&self) -> usize { self.0 }
    fn to(&self) -> usize { self.1 }
    fn weight(&self) -> &T { &self.2 }
    fn into_tuple(self) -> (usize, usize, T) { self }
  }
  impl<T> Arc<T> for &(usize, usize, T) where T: Clone {
    fn from(&self) -> usize { self.0 }
    fn to(&self) -> usize { self.1 }
    fn weight(&self) -> &T { &self.2 }
    fn into_tuple(self) -> (usize, usize, T) { self.clone() }
  }
}

use rolling_hash::{ModIntM61, RollingHash, RollingHashedString};
pub mod rolling_hash {
  use std::{ops::*, cell::*, thread_local};
  use rand::prelude::*;
  
  pub const MOD: u64 = (1 << 61) - 1;
  
  thread_local! {
    static BASE: Cell<ModIntM61> = Cell::new(ModIntM61::from(0));
    static POW: UnsafeCell<Vec<ModIntM61>> = UnsafeCell::new(vec![ModIntM61::from(1)]);
    static POW_INV: UnsafeCell<Vec<ModIntM61>> = UnsafeCell::new(vec![ModIntM61::from(1)]);
  }
  
  const POW_THRESH: usize = 10_000_000;
  
  #[inline]
  fn add(x: u64, y: u64) -> u64 { let mut z = x + y; if z >= MOD { z -= MOD; } z }
  
  #[inline]
  fn sub(x: u64, y: u64) -> u64 { if x < y { x + MOD - y } else { x - y } }
  
  #[inline]
  fn mul(x: u64, y: u64) -> u64 {
    let (x0, x31) = (x & ((1 << 31) - 1), x >> 31);
    let (y0, y31) = (y & ((1 << 31) - 1), y >> 31);
    let t31 = x0 * y31 + x31 * y0;
    let (u0, u31) = (t31 >> 30, t31 & ((1 << 30) - 1));
    let r = x0 * y0 + (x31 * y31 << 1) + u0 + (u31 << 31);
    add(r & ((1 << 61) - 1), r >> 61)
  }
  
  fn pow(mut x: u64, mut y: u64) -> u64 {
    let mut r = 1;
    while y > 0 {
      if y & 1 == 1 {
        r = mul(r, x);
      }
      x = mul(x, x);
      y >>= 1;
    }
    r
  }
  
  #[inline]
  fn inv(x: u64) -> u64 { pow(x, MOD - 2) }
  
  #[inline]
  fn rem(x: u64) -> u64 { add(x & MOD, x >> 61) }
  
  #[derive(Clone, Copy, Debug, Hash)]
  pub struct ModIntM61(u64);
  impl ModIntM61 {
    pub fn raw(x: u64) -> Self { debug_assert!(x < MOD); Self(x) }
    pub fn value(self) -> u64 { self.0 }
    pub fn pow(self, e: u64) -> Self { Self(pow(self.0, e)) }
    pub fn inv(self) -> Self { Self(inv(self.0)) }
  }
  
  macro_rules! impl_from {
    (|$x:ident : $T:ty| { $y:expr }) => {
      impl From<$T> for ModIntM61 {
        fn from($x: $T) -> Self { Self($y) }
      }
    }
  }
  impl_from!(|x: char| { x as u64 });
  impl_from!(|x: u8| { x as u64 });
  impl_from!(|x: u16| { x as u64 });
  impl_from!(|x: u32| { x as u64 });
  impl_from!(|x: u64| { rem(x) });
  impl_from!(|x: u128| { (x % MOD as u128) as u64 });
  impl_from!(|x: i8| { if x < 0 { MOD - (-x) as u64 } else { x as u64 } });
  impl_from!(|x: i16| { if x < 0 { MOD - (-x) as u64 } else { x as u64 } });
  impl_from!(|x: i32| { if x < 0 { MOD - (-x) as u64 } else { x as u64 } });
  impl_from!(|x: i64| { x.rem_euclid(MOD as i64) as u64 });
  impl_from!(|x: i128| { x.rem_euclid(MOD as i128) as u64 });
  
  impl std::str::FromStr for ModIntM61 {
    type Err = <i64 as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> { i64::from_str(s).map(Self::from) }
  }
  
  impl std::fmt::Display for ModIntM61 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_fmt(format_args!("{}", self.0)) }
  }
  
  impl<T: Into<ModIntM61>> Add<T> for ModIntM61 {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output { Self(add(self.0, rhs.into().0)) }
  }
  impl<T: Into<ModIntM61>> Sub<T> for ModIntM61 {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output { Self(sub(self.0, rhs.into().0)) }
  }
  impl<T: Into<ModIntM61>> Mul<T> for ModIntM61 {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output { Self(mul(self.0, rhs.into().0)) }
  }
  impl<T: Into<ModIntM61>> Div<T> for ModIntM61 {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output { Self(mul(self.0, inv(rhs.into().0))) }
  }
  impl<T: Copy + Into<ModIntM61>> PartialEq<T> for ModIntM61 {
    fn eq(&self, other: &T) -> bool { self.0 == (*other).into().0 }
  }
  impl Eq for ModIntM61 {}
  
  fn base() -> ModIntM61 {
    BASE.with(|base| {
      while base.get() == 0 {
        base.set(ModIntM61::from(thread_rng().next_u64()));
      }
      base.get()
    })
  }
  
  fn cell_pow(cell: &UnsafeCell<Vec<ModIntM61>>, n: usize) -> ModIntM61 {
    let pow = unsafe { &mut *cell.get() };
    if n as usize >= pow.len() {
      let next_len = (n + 1).next_power_of_two() as usize;
      let base = base();
      for i in pow.len() .. next_len {
        pow.push(pow[i - 1] * base);
      }
    }
    pow[n as usize]
  }
  
  fn base_pow(n: usize) -> ModIntM61 {
    if n < POW_THRESH {
      POW.with(|cell| cell_pow(cell, n) )
    } else {
      base().pow(n as u64)
    }
  }
  
  fn base_inv_pow(n: usize) -> ModIntM61 {
    if n < POW_THRESH {
      POW_INV.with(|cell| cell_pow(cell, n) )
    } else {
      base().pow((-(n as i128)).rem_euclid(MOD as i128 - 2) as u64)
    }
  }
  
  #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
  pub struct RollingHash {
    value: ModIntM61,
    len: usize,
  }
  impl RollingHash {
    pub fn empty() -> Self { Self { value: ModIntM61::from(0), len: 0 } }
    pub fn new(value: ModIntM61, len: usize) -> Self { Self { value, len } }
    pub fn value(self) -> ModIntM61 { self.value }
    pub fn len(self) -> usize { self.len }
    pub fn concat(self, rhs: Self) -> Self { Self::new(self.value * base_pow(rhs.len) + rhs.value, self.len + rhs.len) }
    pub fn remove_prefix(self, prefix: Self) -> Self { Self::new(self.value - prefix.value * base_pow(self.len - prefix.len), self.len - prefix.len) }
    pub fn remove_suffix(self, suffix: Self) -> Self { Self::new((self.value - suffix.value) * base_inv_pow(suffix.len), self.len - suffix.len) }
  }
  
  impl<T: Into<ModIntM61>> std::iter::FromIterator<T> for RollingHash {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
      let mut value = ModIntM61::from(0);
      let mut len = 0;
      let base = base();
      for x in iter {
        value = value * base + x.into();
        len += 1;
      }
      Self::new(value, len)
    }
  }

  #[derive(Debug, Clone)]
  pub struct RollingHashedString {
    prefixes: Vec<ModIntM61>,
  }
  impl RollingHashedString {
    pub fn slice(&self, range: impl std::ops::RangeBounds<usize>) -> RollingHash {
      use std::ops::Bound::*;
      let l = match range.start_bound() { Included(&l) => l, Excluded(&l) => l + 1, Unbounded => 0 };
      let r = match range.end_bound() { Included(&r) => r + 1, Excluded(&r) => r, Unbounded => self.len() };
      RollingHash::new(self.prefixes[r] - self.prefixes[l] * base_pow(r - l), r - l)
    }

    pub fn len(&self) -> usize { self.prefixes.len() - 1 }
  }
  impl<T: Into<ModIntM61>> std::iter::FromIterator<T> for RollingHashedString {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
      let values = iter.into_iter();
      let mut prefixes = Vec::with_capacity(values.size_hint().0 + 1);
      let mut hash = ModIntM61::from(0);
      prefixes.push(hash);
      let base = base();
      for value in values {
        hash = hash * base + value;
        prefixes.push(hash);
      }
      Self { prefixes }
    }
  }
}

