#[derive(Debug, Clone)]
/// 基数ヒープ（昇順）
pub struct RadixHeapUsize {
  v: Vec<Vec<usize>>,
  last: usize,
  size: usize,
}
impl RadixHeapUsize {
  pub fn new() -> Self {
    Self { v: vec![vec![]; 0usize.leading_zeros() as usize + 1], last: 0, size: 0 }
  }

  pub fn push(&mut self, x: usize) {
    assert!(self.last <= x);
    self.size += 1;
    self.v[Self::bsr(x ^ self.last)].push(x);
  }

  pub fn pop(&mut self) -> Option<usize> {
    if self.size == 0 {
      return None;
    }
    if self.v[0].is_empty() {
      let mut i = 1;
      while self.v[i].is_empty() {
        i += 1;
      }
      let new_last = *self.v[i].iter().min().unwrap();
      unsafe {
        let v = &mut self.v as *mut Vec<Vec<usize>>;
        for &x in (& *v)[i].iter() {
          (&mut *v)[Self::bsr(x ^ new_last)].push(x);
        }
      }
      self.last = new_last;
      self.v[i].clear();
    }
    self.size -= 1;
    self.v[0].pop();
    Some(self.last)
  }

  fn bsr(x: usize) -> usize {
    (0usize.leading_zeros() - x.leading_zeros()) as usize
  }
}
