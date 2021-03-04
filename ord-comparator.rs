struct OrdBy<'a, T, F> { value: T, comparator: &'a F }
impl<'a, T, F: Fn(&T, &T) -> std::cmp::Ordering> OrdBy<'a, T, F> {
  fn new(value: T, comparator: &'a F) -> Self {
    Self { value, comparator }
  }
}
impl<'a, T, F: Fn(&T, &T) -> std::cmp::Ordering> Ord for OrdBy<'a, T, F> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    (&self.comparator)(&self.value, &other.value)
  }
}
impl<'a, T, F: Fn(&T, &T) -> std::cmp::Ordering> PartialOrd for OrdBy<'a, T, F> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
impl<'a, T, F: Fn(&T, &T) -> std::cmp::Ordering> Eq for OrdBy<'a, T, F> {}
impl<'a, T, F: Fn(&T, &T) -> std::cmp::Ordering> PartialEq for OrdBy<'a, T, F> {
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other) == std::cmp::Ordering::Equal
  }
}
