struct SWAG<T> { front: Vec<(T, T)>, back: Vec<T>, id: T, op: fn(T, T) -> T }
impl<T: Copy> SWAG<T> {
	fn new(id: T, op: fn(T, T) -> T) -> Self { SWAG { front: Vec::new(), back: Vec::new(), id, op } }
	fn push(&mut self, x: T) { self.front.push((x, (self.op)(self.front.last().map_or(self.id, |&b| b.1 ), x))); }
	fn pop(&mut self) { if self.back.is_empty() { while let Some(x) = self.front.pop() { self.back.push((self.op)(x.1, *self.back.last().unwrap_or(&self.id))); } } self.back.pop(); }
	fn peek(&self) -> T { (self.op)(*self.back.last().unwrap_or(&self.id), self.front.last().map_or(self.id, |&b| b.1 )) }
}
