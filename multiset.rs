struct BTreeMultiset<E>(std::collections::BTreeMap<E, usize>);
impl<E: Ord> BTreeMultiset<E> {
	pub fn new() -> Self { Multiset(std::collections::BTreeMap::new()) }
	pub fn contains(&self, item: &E) -> bool { self.0.contains_key(item) }
	pub fn count(&self, item: &E) -> usize { self.0.get(item).copied().unwrap_or(0) }
	pub fn add(&mut self, item: E) { *self.0.entry(item).or_insert(0) += 1; }
	pub fn remove(&mut self, item: &E) { if let Some(v) = self.0.get_mut(item) { *v -= 1; if *v < 1 { self.0.remove(item); } } }
	pub fn min(&self) -> Option<&E> { self.0.keys().next() }
	pub fn max(&self) -> Option<&E> { self.0.keys().next_back() }
}

struct HashMultiset<E>(std::collections::HashMap<E, usize>);
impl<E: Eq + Hash> BTreeMultiset<E> {
	pub fn new() -> Self { Multiset(std::collections::HashMap::new()) }
	pub fn contains(&self, item: &E) -> bool { self.0.contains_key(item) }
	pub fn count(&self, item: &E) -> usize { self.0.get(item).copied().unwrap_or(0) }
	pub fn last(&self) -> Option<&E> { self.0.keys().last() }
	pub fn add(&mut self, item: E) { *self.0.entry(item).or_insert(0) += 1; }
	pub fn remove(&mut self, item: &E) { if let Some(v) = self.0.get_mut(item) { *v -= 1; if *v < 1 { self.0.remove(item); } } }
}
