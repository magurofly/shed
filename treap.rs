fn main() {
    let mut treap = ["a", "b", "c", "d", "e", "f", "g", "h"].into_iter().map(|s| [s.to_string(), s.to_string()] ).collect::<Treap<H>>();
    treap.reverse_range(3 .. 6);
    eprintln!("{:?}", treap);
    eprintln!("{:?}", treap.prod(0 .. 8));
}

struct H;
impl TreapHelper for H {
    type S = [String; 2];
    fn op(x: &Self::S, y: &Self::S) -> Self::S { [x[0].to_string() + &y[0], y[1].to_string() + &x[1]] }
    fn e() -> Self::S { ["".to_string(), "".to_string()] }
    type F = ();
}

pub use treap::{Treap, Helper as TreapHelper};
pub mod treap {
    pub trait Helper {
        /// 値の型
        type S: Clone;
        /// 作用の型
        type F: Clone;
        /// 二項演算
        fn op(x: &Self::S, y: &Self::S) -> Self::S;
        /// 単位元
        fn e() -> Self::S;
        /// 左右反転したときの値（二項演算が非可換なとき、 `op((xf, xb), (yf, yb)) = (op(xf, yf), op(yb, xb))` と定義して `rev((f, b)) = (b, f)` とするとよい
        fn rev(x: &Self::S) -> Self::S { x.clone() }
        /// 作用の適用
        fn map(_f: &Self::F, x: &Self::S) -> Self::S { x.clone() }
        /// 作用の合成
        fn compose(f: &Self::F, _g: &Self::F) -> Self::F { f.clone() }
    }

    #[derive(Clone)]
    pub struct Treap<H: Helper> {
        root: RefCell<Option<Box<Node<H>>>>,
    }
    impl<H: Helper> Treap<H> {
        pub fn new() -> Self {
            Self { root: RefCell::new(None) }
        }

        pub fn is_empty(&self) -> bool {
            self.root.borrow().is_none()
        }

        pub fn len(&self) -> usize {
            self.root.borrow().as_ref().map(|root| root.len() ).unwrap_or(0)
        }

        pub fn get(&self, index: usize) -> H::S {
            assert!(index < self.len());
            let mut root = self.root.borrow_mut();
            root.as_mut().expect("list is empty").get(index).value.clone()
        }

        pub fn set(&mut self, index: usize, value: H::S) {
            assert!(index < self.len());
            self.root.get_mut().as_mut().expect("list is empty").mutate(index, |node| { node.value = value; });
        }

        pub fn split_off(&mut self, index: usize) -> Self {
            assert!(index <= self.len());
            let [l, r] = Node::split(self.root.get_mut().take(), index);
            *self.root.get_mut() = l;
            Self { root: RefCell::new(r) }
        }

        pub fn prepend(&mut self, mut front: Self) {
            let back = self.root.get_mut().take();
            *self.root.get_mut() = Node::merge(front.root.get_mut().take(), back);
        }

        pub fn append(&mut self, mut back: Self) {
            let front = self.root.get_mut().take();
            *self.root.get_mut() = Node::merge(front, back.root.get_mut().take());
        }

        pub fn pop_front(&mut self) -> Option<H::S> {
            if self.is_empty() {
                return None;
            }
            let [front, back] = Node::split(self.root.get_mut().take(), 1);
            *self.root.get_mut() = back;
            front.map(|node| node.into_value() )
        }

        pub fn pop_back(&mut self) -> Option<H::S> {
            if self.is_empty() {
                return None;
            }
            let [front, back] = Node::split(self.root.get_mut().take(), self.len() - 1);
            *self.root.get_mut() = front;
            back.map(|node| node.into_value() )
        }

        pub fn push_front(&mut self, x: H::S) {
            let back = self.root.get_mut().take();
            *self.root.get_mut() = Node::merge(Some(Node::new(x)), back);
        }

        pub fn push_back(&mut self, x: H::S) {
            let front = self.root.get_mut().take();
            *self.root.get_mut() = Node::merge(front, Some(Node::new(x)));
        }

        pub fn insert(&mut self, index: usize, x: H::S) {
            assert!(index <= self.len());
            let [front, back] = Node::split(self.root.get_mut().take(), index);
            *self.root.get_mut() = Node::merge(Node::merge(front, Some(Node::new(x))), back);
        }

        pub fn reverse(&mut self) {
            if let Some(root) = self.root.get_mut().as_mut() {
                root.reverse();
            }
        }

        pub fn reverse_range(&mut self, range: impl RangeBounds<usize>) {
            use std::ops::Bound::*;
            let l = match range.start_bound() { Included(&l) => l, Excluded(&l) => l.saturating_sub(1), Unbounded => 0 };
            let r = match range.end_bound() { Included(&r) => r + 1, Excluded(&r) => r, Unbounded => self.len() };
            assert!(l <= r && r <= self.len());
            let [middle, back] = Node::split(self.root.get_mut().take(), r);
            let [front, mut middle] = Node::split(middle, l);
            if let Some(middle) = middle.as_mut() {
                middle.reverse();
            }
            *self.root.get_mut() = Node::merge(front, Node::merge(middle, back));
        }

        pub fn apply(&mut self, index: usize, f: &H::F) {
            self.root.get_mut().as_mut().expect("list is empty").mutate(index, |node| { node.lazy = Node::<H>::compose_lazy(node.lazy.as_ref(), Some(f)); });
        }

        pub fn apply_range(&mut self, range: impl RangeBounds<usize>, f: &H::F) {
            use std::ops::Bound::*;
            let l = match range.start_bound() { Included(&l) => l, Excluded(&l) => l.saturating_sub(1), Unbounded => 0 };
            let r = match range.end_bound() { Included(&r) => r + 1, Excluded(&r) => r, Unbounded => self.len() };
            assert!(l <= r && r <= self.len());
            self.root.get_mut().as_mut().expect("list is empty").apply_range(l, r, f);
        }

        pub fn prod(&mut self, range: impl RangeBounds<usize>) -> H::S {
            use std::ops::Bound::*;
            let l = match range.start_bound() { Included(&l) => l, Excluded(&l) => l.saturating_sub(1), Unbounded => 0 };
            let r = match range.end_bound() { Included(&r) => r + 1, Excluded(&r) => r, Unbounded => self.len() };
            let mut root = self.root.borrow_mut();
            root.as_mut().expect("list is empty").range_prod(l, r)
        }
    }
    impl<H: Helper> std::fmt::Debug for Treap<H> where H::S: std::fmt::Debug {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use std::fmt::*;
            f.write_char('[')?;
            f.write_str(&(0 .. self.len()).map(|i| format!("{:?}", self.get(i)) ).collect::<Vec<_>>().join(", "))?;
            f.write_char(']')
        }
    }
    impl<H: Helper> From<Vec<H::S>> for Treap<H> {
        fn from(vec: Vec<H::S>) -> Self {
            Self::from(&vec[..])
        }
    }
    impl<H: Helper> From<&[H::S]> for Treap<H> {
        fn from(slice: &[H::S]) -> Self {
            fn to_treap<H: Helper>(slice: &[H::S]) -> Option<Box<Node<H>>> {
                if slice.is_empty() {
                    return None;
                }
                if slice.len() == 1 {
                    return Some(Node::new(slice[0].clone()));
                }
                let half = slice.len() / 2;
                let l = to_treap(&slice[.. half]);
                let r = to_treap(&slice[half ..]);
                Node::merge(l, r)
            }
            Self { root: RefCell::new(to_treap(slice)) }
        }
    }
    impl<H: Helper> FromIterator<H::S> for Treap<H> {
        fn from_iter<T: IntoIterator<Item = H::S>>(iter: T) -> Self {
            let mut this = Self::new();
            for x in iter {
                this.push_back(x);
            }
            this
        }
    }

    #[derive(Clone)]
    pub struct Node<H: Helper> {
        child: [Option<Box<Node<H>>>; 2],
        priority: u32,
        len: usize,
        rev: bool,
        value: H::S,
        prod: H::S,
        lazy: Option<H::F>,
    }
    impl<H: Helper> Node<H> {
        pub fn new(value: H::S) -> Box<Self> {
            Box::new(Self {
                child: [None, None],
                priority: rand(),
                len: 1,
                rev: false,
                prod: value.clone(),
                value,
                lazy: None,
            })
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn into_value(self) -> H::S {
            self.value
        }

        pub fn prod(&self) -> H::S {
            if let Some(lazy) = &self.lazy {
                H::map(lazy, &self.prod)
            } else {
                self.prod.clone()
            }
        }

        pub fn merge(l: Option<Box<Self>>, r: Option<Box<Self>>) -> Option<Box<Self>> {
            if l.is_none() || r.is_none() {
                return l.or(r);
            }
            let mut l = l.unwrap();
            l.push();
            let mut r = r.unwrap();
            r.push();
            if l.priority > r.priority {
                l.child[1] = Self::merge(l.child[1].take(), Some(r));
                l.update();
                Some(l)
            } else {
                r.child[0] = Self::merge(Some(l), r.child[0].take());
                r.update();
                Some(r)
            }
        }

        pub fn split(node: Option<Box<Self>>, index: usize) -> [Option<Box<Self>>; 2] {
            if node.is_none() || index == 0 {
                return [None, node];
            }
            let mut node = node.unwrap();
            if index >= node.len {
                return [Some(node), None];
            }
            node.push();
            let left_len = node.child[0].as_ref().map(|left| left.len ).unwrap_or(0);
            if index <= left_len {
                let [l, m] = Self::split(node.child[0].take(), index);
                node.child[0] = m;
                node.update();
                [l, Some(node)]
            } else {
                let [m, r] = Self::split(node.child[1].take(), index - left_len - 1);
                node.child[1] = m;
                node.update();
                [Some(node), r]
            }
        }

        pub fn reverse(&mut self) {
            self.rev ^= true;
        }

        pub fn get(&mut self, index: usize) -> &Self {
            self.push();
            let left_len = self.child[0].as_mut().map(|left| left.len ).unwrap_or(0);
            if index < left_len {
                self.child[0].as_mut().unwrap().get(index)
            } else if index == left_len {
                self
            } else {
                self.child[1].as_mut().unwrap().get(index - left_len - 1)
            }
        }

        pub fn mutate(&mut self, index: usize, f: impl FnOnce(&mut Self)) {
            self.push();
            let left_len = self.child[0].as_ref().map(|left| left.len ).unwrap_or(0);
            if index < left_len {
                self.child[0].as_mut().unwrap().mutate(index, f);
            } else if index == left_len {
                f(self);
            } else {
                self.child[1].as_mut().unwrap().mutate(index - left_len - 1, f)
            }
            self.update();
        }

        pub fn range_prod(&mut self, l: usize, r: usize) -> H::S {
            if l >= r {
                return H::e();
            }
            self.push();
            if l == 0 && self.len <= r {
                return self.prod.clone();
            }
            let left_len = self.child[0].as_ref().map(|left| left.len ).unwrap_or(0);
            let mut prod = H::e();
            if l < left_len {
                prod = H::op(&self.child[0].as_mut().unwrap().range_prod(l, r.min(left_len)), &prod);
            }
            if l <= left_len && left_len + 1 <= r {
                prod = H::op(&prod, &self.prod);
            }
            if left_len + 1 < r {
                prod = H::op(&prod, &self.child[1].as_mut().unwrap().range_prod(l.saturating_sub(left_len + 1), r.saturating_sub(left_len + 1)));
            }
            prod
        }

        pub fn apply_range(&mut self, l: usize, r: usize, f: &H::F) {
            if l >= r {
                return;
            }
            self.push();
            if l == 0 && self.len <= r {
                self.lazy = Self::compose_lazy(self.lazy.as_ref(), Some(f));
                return;
            }
            let left_len = self.child[0].as_ref().map(|left| left.len ).unwrap_or(0);
            if l < left_len {
                self.child[0].as_mut().unwrap().apply_range(l, r.min(left_len), f);
            }
            if l <= left_len && left_len + 1 <= r {
                self.value = H::map(f, &self.value);
            }
            if left_len + 1 < r {
                self.child[1].as_mut().unwrap().apply_range(l.saturating_sub(left_len + 1), r.saturating_sub(left_len + 1), f);
            }
            self.update();
        }

        pub fn update(&mut self) {
            let mut len = 1;
            let mut prod = self.value.clone();
            if let Some(left) = self.child[0].as_ref() {
                prod = H::op(&left.prod(), &prod);
                len += left.len;
            }
            if let Some(right) = self.child[1].as_ref() {
                prod = H::op(&prod, &right.prod());
                len += right.len;
            }
            self.prod = prod;
            self.len = len;
        }

        pub fn push(&mut self) {
            if self.rev || self.lazy.is_some() {
                let rev = std::mem::replace(&mut self.rev, false);
                let lazy = self.lazy.take();
                if rev {
                    self.value = H::rev(&self.value);
                    self.prod = H::rev(&self.prod);
                    self.child.swap(0, 1);
                }
                if let Some(lazy) = &lazy {
                    self.value = H::map(lazy, &self.value);
                    self.prod = H::map(lazy, &self.prod);
                }
                for child in &mut self.child {
                    if let Some(child) = child {
                        child.lazy = Self::compose_lazy(child.lazy.as_ref(), lazy.as_ref());
                        child.rev ^= rev;
                    }
                }
            }
        }

        fn compose_lazy(f: Option<&H::F>, g: Option<&H::F>) -> Option<H::F> {
            if f.is_none() || g.is_none() {
                None
            } else {
                Some(H::compose(f.unwrap(), g.unwrap()))
            }
        }
    }

    fn rand() -> u32 {
        static mut X: u32 = 0x01234567;
        unsafe {
            X ^= X << 13;
            X ^= X >> 17;
            X ^= X << 5;
            X
        }
    }

    use std::{cell::*, ops::RangeBounds};
}
