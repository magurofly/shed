#[derive(Debug, Clone)]
/// ρ形に循環する関数の繰り返し
pub struct Rho<T> {
    list: Vec<T>,
    tail: usize,
}
impl<T> Rho<T> {
    /// x: 初期値, f: 関数
    pub fn new<F>(mut x: T, mut f: F) -> Self
    where
        T: Clone + std::hash::Hash + Eq,
        F: FnMut(T) -> T,
    {
        let mut rev = std::collections::HashMap::new();
        for i in 0.. {
            if rev.contains_key(&x) {
                break;
            }
            rev.insert(x.clone(), i);
            x = (f)(x);
        }
        let tail = rev[&&x];
        let mut list = Vec::with_capacity(rev.len());
        for (l, (y, i)) in rev.into_iter().enumerate() {
            list.push(y);
            if i < l {
                list.swap(i, l);
            }
        }
        Self { list, tail }
    }

    /// サイクルに入るまでの部分
    pub fn tail(&self) -> &[T] {
        &self.list[..self.tail]
    }

    /// サイクルの部分
    pub fn cycle(&self) -> &[T] {
        &self.list[self.tail..]
    }
}
impl<T> std::ops::Index<usize> for Rho<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index < self.list.len() {
            &self.list[index]
        } else {
            let cycle = self.cycle();
            &cycle[(index - self.tail) % cycle.len()]
        }
    }
}
