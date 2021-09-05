#[derive(Clone, Debug)]
pub struct BTreeMultiset<E>(std::collections::BTreeMap<E, usize>);
impl<E: Ord> BTreeMultiset<E> {
	pub fn new() -> Self { Self(std::collections::BTreeMap::new()) }
	pub fn contains(&self, item: &E) -> bool { self.0.contains_key(item) }
	pub fn count(&self, item: &E) -> usize { self.0.get(item).copied().unwrap_or(0) }
	pub fn insert(&mut self, item: E) { *self.0.entry(item).or_insert(0) += 1; }
	pub fn remove(&mut self, item: &E) { if let Some(v) = self.0.get_mut(item) { if *v <= 1 { self.0.remove(item); } else { *v -= 1; } } }
	pub fn min(&self) -> Option<&E> { self.0.keys().next() }
	pub fn max(&self) -> Option<&E> { self.0.keys().next_back() }
	pub fn lower_bound(&self, min: E) -> Option<&E> { self.0.range(min ..).next() }
	pub fn upper_bound(&self, min: E) -> Option<&E> { use std::ops::Bound::*; self.0.range((Excluded(min), Unbounded)).next() }
}

#[derive(Clone, Debug)]
pub struct HashMultiset<E>(std::collections::HashMap<E, usize>);
impl<E: Eq + Hash> BTreeMultiset<E> {
	pub fn new() -> Self { Self(std::collections::HashMap::new()) }
	pub fn contains(&self, item: &E) -> bool { self.0.contains_key(item) }
	pub fn count(&self, item: &E) -> usize { self.0.get(item).copied().unwrap_or(0) }
	pub fn last(&self) -> Option<&E> { self.0.keys().last() }
	pub fn insert(&mut self, item: E) { *self.0.entry(item).or_insert(0) += 1; }
	pub fn remove(&mut self, item: &E) { if let Some(v) = self.0.get_mut(item) { if *v <= 1 { self.0.remove(item); } else { *v -= 1; } } }
}
