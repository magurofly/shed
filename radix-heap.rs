#[derive(Debug, Clone)]
/// 基数ヒープ（昇順）
pub struct RadixHeap<T> {
  v: Vec<Vec<(usize, T)>>,
  last: usize,
  size: usize,
}
impl<T> RadixHeap<T> {
  pub fn new() -> Self {
    let mut v = vec![];
    for _ in 0 ..= 0usize.leading_zeros() {
      v.push(vec![]);
    }
    Self { v, last: 0, size: 0 }
  }

  pub fn is_empty(&self) -> bool {
    self.size == 0
  }

  pub fn push(&mut self, key: usize, value: T) {
    assert!(self.last <= key);
    self.size += 1;
    self.v[Self::bsr(key ^ self.last)].push((key, value));
  }

  pub fn pop(&mut self) -> Option<(usize, T)> {
    if self.size == 0 {
      return None;
    }
    if self.v[0].is_empty() {
      let mut i = 1;
      while self.v[i].is_empty() {
        i += 1;
      }
      let new_last = self.v[i].iter().map(|x| x.0).min().unwrap();
      unsafe {
        let v = &mut self.v as *mut Vec<Vec<(usize, T)>>;
        for pair in (&mut *v)[i].drain(..) {
          (&mut *v)[Self::bsr(pair.0 ^ new_last)].push(pair);
        }
      }
      self.last = new_last;
    }
    self.size -= 1;
    self.v[0].pop()
  }

  fn bsr(x: usize) -> usize {
    (0usize.leading_zeros() - x.leading_zeros()) as usize
  }
}
