// 2の倍数を飛ばす
pub struct ErathosthenesSieve {
  n: usize,
  sieve: Vec<bool>,
}
impl ErathosthenesSieve {
  pub fn new(n: usize) -> Self {
    let mut this = Self { n: n + 1, sieve: vec![true; n + 1] };
    this.init();
    this
  }

  pub fn is_prime(&self, n: usize) -> bool {
    assert!(n < self.n);
    n > 1 && (n == 2 || n % 2 != 0 && self.sieve[n / 2])
  }

  pub fn primes(&self) -> Vec<usize> {
    let mut primes = vec![];
    if self.n >= 2 { primes.push(2); }
    for i in  (3 .. self.n).step_by(2) {
      if self.is_prime(i) {
        primes.push(i);
      }
    }
    primes
  }

  fn init(&mut self) {
    self.sieve[0] = false;
    let sqrt_x = ((self.n as f64).sqrt() + 0.1).ceil() as usize;
    let sqrt_xi = sqrt_x / 2;
    for i in 1 .. sqrt_xi {
      if !self.sieve[i] { continue; }
      let p = 2 * i + 1;
      for mult in (2 * i * (i + 1) .. self.n).step_by(p) {
        self.sieve[mult] = false;
      }
    }
  }
}
