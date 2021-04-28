struct Imos2D<T, A, S> {
  n: usize, m: usize,
  imos: Vec<Vec<T>>,
  id: T, add: A, sub: S
}
impl<T: Copy, A: Fn(T, T) -> T, S: Fn(T, T) -> T> Imos2D<T, A, S> {
  fn new(n: usize, m: usize, mat: &Vec<Vec<T>>, id: T, add: A, sub: S) -> Self {
    let mut imos = vec![vec![id; m + 1]; n + 1];
    for i in 0 .. n {
      for j in 0 .. m {
        imos[i + 1][j + 1] = (sub)((add)((add)(imos[i + 1][j], imos[i][j + 1]), mat[i][j]), imos[i][j]);
      }
    }
    Self { n, m, imos, id, add, sub }
  }
  
  fn at(&self, i: usize, j: usize) -> T {
    self.rect(i, j, i + 1, j + 1)
  }
  
  fn rect(&self, i1: usize, j1: usize, i2: usize, j2: usize) -> T {
    (self.sub)((self.add)(self.imos[i2][j2], self.imos[i1][j1]), (self.add)(self.imos[i1][j2], self.imos[i2][j1]))
  }
}
