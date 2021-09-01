/// 線形合同法 0以外の何かで初期化する
struct Rng(u64);
impl Rng {
  pub fn next_u32(&mut self) -> u32 {
    self.0 = 48271 * self.0 % 2147483647;
    self.0 as u32
  }
}

/// Seidel's LP Algorithm
/// https://niuez.github.io/cp-cpp-library/math/seidels_lp.html
/// find x = {x[0], x[1], ..., x[d - 1]}
/// maximizes (0 .. d).map(|i| c[i] * x[i]).sum()
fn seidel_lp(rnd: &mut Rng, d: usize, c: &[f64], mat: &[Vec<f64>], bounds: &[(f64, f64)]) -> Option<Vec<f64>> {
  // eprintln!("seidel_lp(rnd, d = {}, c = {:?}, mat = {:?}, bounds = {:?})", d, c, mat, bounds);
  assert_eq!(c.len(), d, "c.len() != d");
  let m = mat.len();
  for i in 0 .. m {
    assert_eq!(mat[i].len(), d + 1, "mat[{}].len() != d + 1 ({}); mat = {:?}", i, d + 1, &mat);
  }
  fn eps_eq(x: f64, y: f64) -> bool {
    (x - y).abs() <= 1e-9
  }

  if d == 1 {
    let (mut low, mut high) = bounds[0];
    let mut z = 0.0;
    for a in mat.iter() {
      if eps_eq(a[0], 0.0) {
        if eps_eq(a[1], z) || a[1] < z {
          z = a[1];
        }
      } else if a[0] > 0.0 {
        // greater
        // high = high.min(a[1] / a[0]);
        // 意味なし？
        let pa = a[1] / a[0];
        if eps_eq(pa, high) || pa < high {
          high = pa;
        }
      } else {
        // less
        // low = low.max(a[1] / a[0]);
        let pa = a[1] / a[0];
        if eps_eq(pa, low) || pa > low {
          low = pa;
        }
      }
    }
    if z.is_sign_negative() || high < low {
      None
    } else if eps_eq(c[0], 0.0) || c[0] > 0.0 {
      Some(vec![high])
    } else {
      Some(vec![low])
    }
  } else if m == 0 {
    // no constraints
    Some((0 .. d).map(|i| {
      if eps_eq(c[i], 0.0) || c[i] > 0.0 {
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
      let value = a.iter().zip(&v).fold(0.0, |sum, (&x, &y)| sum + x * y);
      if value <= a[d] {
        return Some(v);
      }
    }
    let k = (0 .. d).rev().find(|&i| !eps_eq(a[i], 0.0))?;

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
        let x = -(a[k].recip() * a[i]);
        bar_mat[m - 1].push(x);
        bar_mat[m].push(x);
      }
    }
    bar_mat[m - 1].push(bounds[k].1);
    bar_mat[m].push(bounds[k].0);

    // eprintln!("call from {}", line!() + 1);
    v = seidel_lp(rnd, d - 1, &bar_c, &bar_mat, &next_bounds)?;
    v.insert(k, 0.0);
    let s = a.iter().zip(&v).fold(0.0, |sum, (&x, &y)| sum + x * y);
    v[k] = (a[d] - s) / a[k];
    Some(v)
  }
}
