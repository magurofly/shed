type ll = i64;
const INF: ll = 1_000_000_000;
// unverified
fn tsp(n: usize, dist: &Vec<Vec<ll>>) -> ll {
  let mut dp = vec![vec![INF; n]; 1 << n];
  for i in 0 .. n { dp[0][i] = 0; }
  for bits in 0 .. (1 << n) - 1 {
    for last in 0 .. n {
      if bits >> last & 1 == 0 { continue; }
      for next in 0 .. n {
        if bits >> next & 1 == 1 { continue; }
        if dp[bits | 1 << next][next] > dp[bits][last] + dist[last][next] {
          dp[bits | 1 << next][next] = dp[bits][last] + dist[last][next];
        }
      }
    }
  }
  
  let ans = INF;
  for last in 0 .. n {
    if ans > dp[(1 << n) - 1][last] {
      ans = dp[(1 << n) - 1][last];
    }
  }
  
  return ans;
}
