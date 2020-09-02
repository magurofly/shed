struct Multiset<E>(std::collections::BTreeMap<E, usize>);
impl<E: Ord> Multiset<E> {
	pub fn new() -> Self { Multiset(std::collections::BTreeMap::new()) }
	pub fn contains(&self, item: &E) -> bool { self.0.contains_key(item) }
	pub fn last(&self) -> Option<&E> { self.0.keys().last() }
	pub fn add(&mut self, item: E) { *self.0.entry(item).or_insert(0) += 1; }
	pub fn remove(&mut self, item: &E) { if let Some(v) = self.0.get_mut(item) { *v -= 1; if *v < 1 { self.0.remove(item); } } }
}
