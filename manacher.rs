fn manacher(s: &[char]) -> Vec<usize> {
  let mut radius = vec![0; s.len()];
  let mut i = 0;
  let mut j = 0;
  while i < s.len() {
    while i >= j && i + j < s.len() && s[i - j] == s[i + j] {
      j += 1;
    }
    radius[i] = j;
    let mut k = 1;
    while i >= k && i + k < s.len() && k + radius[i - k] < j {
      radius[i + k] = radius[i - k];
      k += 1;
    }
    i += k;
    j -= k;
  }
  radius
}
