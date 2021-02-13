use num_traits::*;
fn main() {
  let mut f = Factorials::<usize>::new(1_000_000_007);
  println!("{}", f.fact(6));
}



struct Factorials<N> {
  fac: Vec<N>, fin: Vec<N>, inv: Vec<N>, m: N, limit: N,
}

impl<N: PrimInt> Factorials<N> {
  fn new(modulo: N) -> Self {
    Self {fac: vec![N::one(), N::one()], fin: vec![N::one(), N::one()], inv: vec![N::zero(), N::one()], m: modulo, limit: N::one()}
  }

  fn expand(&mut self, limit: N) {
    if self.limit >= limit { return; }
    for i in self.limit.to_usize().unwrap() + 1 ..= limit.to_usize().unwrap() {
      self.fac.push(self.fac[i - 1] * N::from(i).unwrap() % self.m);
      self.inv.push(self.m - self.inv[self.m.to_usize().unwrap() % i] * (self.m / N::from(i).unwrap()) % self.m);
      self.fin.push(self.fin[i - 1] * self.inv[i] % self.m);
    }
    self.limit = limit;
  }

  fn inv(&mut self, n: N) -> N { self.expand(n); self.inv[n.to_usize().unwrap()] }
  fn fact_inv(&mut self, n: N) -> N { self.expand(n); self.fin[n.to_usize().unwrap()] }
  fn fact(&mut self, n: N) -> N { self.expand(n); self.fac[n.to_usize().unwrap()] }
  fn comb(&mut self, n: N, k: N) -> N { self.perm(n, k) * self.fact_inv(k) % self.m }
  fn perm(&mut self, n: N, k: N) -> N { self.fact(n) * self.fact_inv(n - k) % self.m }
}
