#![allow(non_snake_case)]

fn main() {
  use proconio::*;

  eprintln!(r#"Seidel LP Solver (thanks Niu)
max.  c^T x
s.t.  A x <= b
      l <= x <= r
Input format:
  d m
  c[1] c[2] ... c[d]
  A[1, 1] A[1, 2] ... A[1, d] b[1]
  A[2, 1] A[2, 2] ... A[2, d] b[2]
  ...
  A[m, 1] A[m, 2] ... A[m, d] b[m]
  l[1] r[1]
  l[2] r[2]
  ...
  l[d] r[d]"#);

  input! {
    d: usize, m: usize,
    c: [f64; d],
    A: [[f64; d + 1]; m],
    bounds: [(f64, f64); d],
  }

  let x = seidel_lp(&mut thread_rng(), d, &c, &A, &bounds).expect("No solution found");

  println!("{}", x.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}


use rand::*;
use num_traits::*;

/// Seidel's LP Algorithm
/// https://niuez.github.io/cp-cpp-library/math/seidels_lp.html
/// find x = {x[0], x[1], ..., x[d - 1]}
/// maximizes (0 .. d).map(|i| c[i] * x[i]).sum()
fn seidel_lp<T: Float + std::fmt::Debug>(rnd: &mut impl RngCore, d: usize, c: &[T], mat: &[Vec<T>], bounds: &[(T, T)]) -> Option<Vec<T>> {
  // eprintln!("seidel_lp(rnd, d = {}, c = {:?}, mat = {:?}, bounds = {:?})", d, c, mat, bounds);
  assert_eq!(c.len(), d, "c.len() != d");
  let m = mat.len();
  for i in 0 .. m {
    assert_eq!(mat[i].len(), d + 1, "mat[{}].len() != d + 1 ({}); mat = {:?}", i, d + 1, &mat);
  }
  fn eps_eq<T: Float>(x: T, y: T) -> bool {
    (x - y).abs() <= T::epsilon()
  }

  if d == 1 {
    let (mut low, mut high) = bounds[0];
    let mut z = T::zero();
    for a in mat.iter() {
      if eps_eq(a[0], T::zero()) {
        if eps_eq(a[1], z) || a[1] < z {
          z = a[1];
        }
      } else if a[0].is_sign_positive() {
        // greater
        high = high.min(a[1] / a[0]);
      } else {
        // less
        low = low.max(a[1] / a[0]);
      }
    }
    if z.is_sign_negative() || high < low {
      None
    } else if eps_eq(c[0], T::zero()) || c[0].is_sign_positive() {
      Some(vec![high])
    } else {
      Some(vec![low])
    }
  } else if m == 0 {
    // no constraints
    Some((0 .. d).map(|i| {
      if eps_eq(c[i], T::zero()) || c[i].is_sign_positive() {
        bounds[i].1
      } else {
        bounds[i].0
      }
    }).collect())
  } else {
    let rmi = rnd.next_u32() as usize % m;
    let a = &mat[rmi];
    let mut next_mat = Vec::with_capacity(m - 1);
    for i in 0 .. m {
      if i != rmi {
        next_mat.push(mat[i].clone());
      }
    }
    // eprintln!("call from {}", line!() + 1);
    let mut v = seidel_lp(rnd, d, c, &next_mat, bounds)?;
    {
      let value = a.iter().zip(&v).fold(T::zero(), |sum, (&x, &y)| sum + x * y);
      if value <= a[d] {
        return Some(v);
      }
    }
    let k = (0 .. d).rev().find(|&i| !eps_eq(a[i], T::zero()))?;

    let mut next_bounds = vec![];
    for i in 0 .. d {
      if i != k {
        next_bounds.push(bounds[i]);
      }
    }
    let mut bar_mat = vec![Vec::with_capacity(d); m + 1];
    for mi in 0 .. m - 1 {
      let ratio = next_mat[mi][k] / a[k];
      for i in 0 ..= d {
        if i != k {
          bar_mat[mi].push(next_mat[mi][i] - ratio * a[i]);
        }
      }
    }
    let mut bar_c = Vec::with_capacity(d - 1);
    {
      let ratio = c[k] / a[k];
      for i in 0 .. d {
        if i != k {
          bar_c.push(c[i] - ratio * a[i]);
        }
      }
    }
    for i in 0 .. d {
      if i != k {
        let x = -a[k].recip() * a[i];
        bar_mat[m - 1].push(x);
        bar_mat[m].push(x);
      }
    }
    bar_mat[m - 1].push(bounds[k].1);
    bar_mat[m].push(bounds[k].0);

    // eprintln!("call from {}", line!() + 1);
    v = seidel_lp(rnd, d - 1, &bar_c, &bar_mat, &next_bounds)?;
    v.insert(k, T::zero());
    let s = a.iter().zip(&v).fold(T::zero(), |sum, (&x, &y)| sum + x * y);
    v[k] = (a[d] - s) / a[k];
    Some(v)
  }
}
