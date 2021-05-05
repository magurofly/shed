// { 素数リスト, 最小素因数のテーブル }
pair<vector<int>, vector<int>> prime_table(int n) {
  vector<int> primes;
  vector<int> table(n + 1);
  for (int d = 2; d <= n; d++) {
    if (!table[d]) {
      table[d] = d;
      primes.push_back(d);
    }
    for (int p : primes) {
      if (p * d > n || p > table[d]) break;
      table[p * d] = p;
    }
  }
  return {primes, table};
}
