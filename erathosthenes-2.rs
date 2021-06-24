use erathosthenes2::ErathosthenesSieve;

fn main() {
  let s = ErathosthenesSieve::new(100);
  println!("{:?}", s.primes());
}

pub mod erathosthenes2 {
  // 2, 3, 5 の倍数を飛ばす
  pub struct ErathosthenesSieve {
    n: usize,
    sieve: Vec<i32>,
  }
  impl ErathosthenesSieve {
    pub fn new(n: usize) -> Self {
      let n = n + 1;
      let mut this = Self { n, sieve: vec![0xFF; n / 30 + (if n % 30 == 0 { 0 } else { 1 })] };
      this.init();
      this
    }

    pub fn is_prime(&self, n: usize) -> bool {
      assert!(n < self.n);
      if n == 3 || n == 5 || n == 7 { return true; }
      if n % 3 == 0 || n % 5 == 0 || n % 7 == 0 { return false; }
      let index = match n % 30 {
        1 => 0,
        7 => 1,
        11 => 2,
        13 => 3,
        17 => 4,
        19 => 5,
        23 => 6,
        _ => 7,
      };
      (self.sieve[n / 30] & 1 << index) != 0
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
      let size = self.sieve.len();
      self.sieve[size - 1] = match self.n % 30 {
        0 | 1 => 0x00,
        2 | 3 | 4 | 5 | 6 | 7 => 0x01,
        8 | 9 | 10 | 11 => 0x03,
        12 | 13 => 0x07,
        14 | 15 | 16 | 17 => 0x0F,
        18 | 19 => 0x1F,
        20 | 21 | 22 | 23 => 0x3F,
        _ => 0x7F,
      };
      self.sieve[0] = 0xFE;
      let sqrt_x = ((self.n as f64).sqrt() + 0.1).ceil() as usize;
      let sqrt_xi = sqrt_x / 30 + 1;
      for i in 0 .. sqrt_xi {
        let mut flags = self.sieve[i];
        while flags != 0 {
          let ibit = flags.trailing_zeros() as usize;
          let m = KMOD30[ibit];
          let pm = 30 * i + 2 * m;
          let mut j = i * pm + (m * m) / 30;
          let mut k = ibit;
          while j < size {
            self.sieve[j] &= KMASK[ibit][k];
            j += i * C1[k] + C0[ibit][k];
            k = (k + 1) & 7;
          }
          flags &= flags - 1;
        }
      }
    }
  }

  const KMOD30: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
  const KMASK: [[i32; 8]; 8] = [
    [0xfe, 0xfd, 0xfb, 0xf7, 0xef, 0xdf, 0xbf, 0x7f],
    [0xfd, 0xdf, 0xef, 0xfe, 0x7f, 0xf7, 0xfb, 0xbf],
    [0xfb, 0xef, 0xfe, 0xbf, 0xfd, 0x7f, 0xf7, 0xdf],
    [0xf7, 0xfe, 0xbf, 0xdf, 0xfb, 0xfd, 0x7f, 0xef],
    [0xef, 0x7f, 0xfd, 0xfb, 0xdf, 0xbf, 0xfe, 0xf7],
    [0xdf, 0xf7, 0x7f, 0xfd, 0xbf, 0xfe, 0xef, 0xfb],
    [0xbf, 0xfb, 0xf7, 0x7f, 0xfe, 0xef, 0xdf, 0xfd],
    [0x7f, 0xbf, 0xdf, 0xef, 0xf7, 0xfb, 0xfd, 0xfe],
  ];
  const C1: [usize; 8] = [6, 4, 2, 4, 2, 4, 6, 2];
  const C0: [[usize; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 1], [1, 1, 1, 0, 1, 1, 1, 1],
    [2, 2, 0, 2, 0, 2, 2, 1], [3, 1, 1, 2, 1, 1, 3, 1],
    [3, 3, 1, 2, 1, 3, 3, 1], [4, 2, 2, 2, 2, 2, 4, 1],
    [5, 3, 1, 4, 1, 3, 5, 1], [6, 4, 2, 4, 2, 4, 6, 1],
  ];
}
