fn main() {
  let mut xs = vec![(3, 0), (2, 1), (6, 2), (1, 3), (3, 4)];
  println!("{:?}", &xs);
  radix_sort_by(&mut xs, 2, |x| x.0 );
  println!("{:?}", &xs);
}

fn radix_sort_by<T>(xs: &mut Vec<T>, bits: usize, mut f: impl FnMut(&T) -> usize) {
  let mut buckets = Vec::new();
  buckets.resize_with(16, Vec::new);
  for d in 0 .. (bits + 3) / 4 {
    for x in xs.drain(..) {
      let i = (f)(&x) >> 4 * d & 15;
      buckets[i].push(x);
    }
    for bucket in &mut buckets {
      xs.append(bucket);
    }
  }
}
