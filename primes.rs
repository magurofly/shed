// template.rs と併せて使うこと前提

// 線形篩により limit 以下の素数のリストと最小の素因数のテーブルを生成する O(limit)
fn linear_sieve(limit: usize) -> (Vec<usize>, Vec<usize>) {
  let mut primes = vec![];
  let mut table = vec![0; limit + 1];
  for d in 2 ..= limit {
    if table[d] == 0 {
      table[d] = d;
      primes.push(d);
    }
    for &p in &primes {
      if p * d > limit || p > table[d] { break; }
      table[p * d] = p;
    }
  }
  (primes, table)
}
  
// 素因数分解する O(sqrt(n))
fn factorize(mut n: i64) -> Vec<i64> {
  let mut factors = vec![];
  let mut k = 2;
  while n > 1 && k * k < n {
    while n % k == 0 {
      factors.push(k);
      n /= k;
    }
    k += 1;
  }
  if n > 1 {
    factors.push(n);
  }
  factors
}

// 拡張ユークリッドの互除法
fn ext_gcd<N: PrimInt>(a: N, b: N, x: &mut N, y: &mut N) -> N {
  if b == N::zero() {
    *x = N::one();
    *y = N::zero();
    return a;
  }
  let q = a / b;
  let g = ext_gcd(b, a - q * b, x, y);
  let z = *x - q * *y;
  *x = *y;
  *y = z;
  g
}
