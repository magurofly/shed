#[derive(Debug, Clone)]
pub struct SumDeque<T> where T: std::ops::Add<Output = T> {
    front: Vec<(T, T)>,
    back: Vec<(T, T)>,
}
impl<T> SumDeque<T> where T: Clone + std::ops::Add<Output = T> {
    pub fn new() -> Self { Self { front: vec![], back: vec![] } }
    pub fn len(&self) -> usize { self.front.len() + self.back.len() }
    pub fn is_empty(&self) -> bool { self.front.is_empty() && self.back.is_empty() }
    pub fn push_back(&mut self, x: T) { let y = Self::merge(Self::second(self.back.last()), Some(x.clone())).unwrap(); self.back.push((x, y)); }
    pub fn push_front(&mut self, x: T) { let y = Self::merge(Self::second(self.front.last()), Some(x.clone())).unwrap(); self.front.push((x, y)); }
    pub fn pop_back(&mut self) -> Option<T> { if self.back.is_empty() { self.back = Self::split(&mut self.front); } Some(self.back.pop()?.0) }
    pub fn pop_front(&mut self) -> Option<T> { if self.front.is_empty() { self.front = Self::split(&mut self.back); } Some(self.front.pop()?.0) }
    pub fn sum(&self) -> Option<T> { Self::merge(Self::second(self.front.last()), Self::second(self.back.last())) }
    fn refresh(a: &mut [(T, T)]) { let mut y = None; for (x, z) in a { y = Self::merge(y, Some(x.clone())); *z = y.clone().unwrap(); } }
    fn split(a: &mut Vec<(T, T)>) -> Vec<(T, T)> { let mut b = a.drain(..= a.len() / 2).collect::<Vec<_>>(); b.reverse(); Self::refresh(a); Self::refresh(&mut b); b }
    fn merge(a: Option<T>, b: Option<T>) -> Option<T> { if let Some(x) = a { Some(if let Some(y) = b { x + y } else { x }) } else { b } }
    fn second(o: Option<&(T, T)>) -> Option<T> { o.map(|p| p.1.clone()) }
}
