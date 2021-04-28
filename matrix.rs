#[derive(Clone, Debug)]
struct Matrix<'a, T: Clone> {
  n: usize, m: usize,
  rows: std::borrow::Cow<'a, Vec<Vec<T>>>,
}
impl<T: Clone> Matrix<'_, T> {
  fn new(rows: Vec<Vec<T>>) -> Self {
    Self { n: rows.len(), m: rows[0].len(), rows: std::borrow::Cow::Owned(rows) }
  }

  fn zero(n: usize, m: usize) -> Self where T: num_traits::Zero {
    Self { n, m, rows: std::borrow::Cow::Owned(vec![vec![T::zero(); m]; n]) }
  }

  fn identity(n: usize) -> Self where T: num_traits::Zero + num_traits::One {
    let mut rows = vec![vec![T::zero(); n]; n];
    for i in 0 .. n {
      rows[i][i] = T::one();
    }
    Self { n, m: n, rows: std::borrow::Cow::Owned(rows) }
  }
  
  fn at(&self, i: usize, j: usize) -> T {
    assert!(i < self.n && j < self.m);
    self.rows[i][j].clone()
  }

  fn pow(&self, mut e: usize) -> Self where T: Clone + std::ops::Add + std::ops::Mul + num_traits::Zero + num_traits::One {
    assert!(self.n == self.m);
    let mut r = Self::identity(self.n);
    let mut a = self.clone();
    while e != 0 {
      if (e & 1) == 1 {
        r = r * a.clone();
      }
      a = a.clone() * a.clone();
      e >>= 1;
    }
    r
  }
}
impl<'a, T: Clone + std::ops::Add<Output = T>> std::ops::Add for Matrix<'a, T> {
  type Output = Matrix<'a, T>;
  fn add(self, other: Matrix<T>) -> Matrix<T> {
    assert!(self.n == other.n && self.m == other.m);
    Matrix {
      n: self.n, m: self.m,
      rows: std::borrow::Cow::Owned((0 .. self.n).map(|i| (0 .. self.m).map(|j| self.at(i, j) + other.at(i, j) ).collect::<Vec<_>>()).collect::<Vec<_>>())
    }
  }
}
impl<'a, T: Clone + std::ops::Add<Output = T> + std::ops::Mul<Output = T>> std::ops::Mul<T> for Matrix<'a, T> {
  type Output = Matrix<'a, T>;
  fn mul(self, c: T) -> Matrix<'a, T> {
    Matrix {
      n: self.n, m: self.m,
      rows: std::borrow::Cow::Owned((0 .. self.n).map(|i| (0 .. self.m).map(|j| self.at(i, j) * c.clone() ).collect::<Vec<_>>()).collect::<Vec<_>>())
    }
  }
}
impl<'a, T: Clone + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + num_traits::Zero> std::ops::Mul for Matrix<'a, T> {
  type Output = Matrix<'a, T>;
  fn mul(self, other: Matrix<'a, T>) -> Matrix<'a, T> {
    assert!(self.m == other.n);
    Matrix {
      n: self.n, m: other.m,
      rows: std::borrow::Cow::Owned((0 .. self.n).map(|i| (0 .. other.m).map(|j| {
        let mut sum = T::zero();
        for k in 0 .. self.m {
          sum = sum + self.at(i, k) * other.at(k, j)
        }
        sum
      }).collect::<Vec<_>>()).collect::<Vec<_>>())
    }
  }
}
