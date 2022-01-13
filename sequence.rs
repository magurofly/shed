fn main() {
  struct Join;
  impl Monoid for Join {
    type S = String;
    fn op(x: &String, y: &String) -> String {
      let mut s = String::new();
      s.push_str(x);
      s.push_str(y);
      s
    }
    fn e() -> String { "".to_string() }
  }

  let mut seq = Sequence::<Join>::new();
  seq.push_back("b".to_string());
  seq.push_front("a".to_string());
  seq.push_back("c".to_string());
  println!("prod(..) = {}", seq.prod(..));
  eprintln!("{:?}", seq);

  seq.print();
}

use sequence::{Monoid, Sequence};
pub mod sequence {
  pub trait Monoid {
    type S;
    fn op(x: &Self::S, y: &Self::S) -> Self::S;
    fn e() -> Self::S;
  }

  pub struct Sequence<M: Monoid> {
    root: Option<Box<Node<M>>>,
  }
  impl<M: Monoid> std::fmt::Debug for Sequence<M> where M::S: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_fmt(format_args!("Sequence({:?})", &self.root))
    }
  }
  impl<M: Monoid> Clone for Sequence<M> where M::S: Clone {
    fn clone(&self) -> Self {
      Self {
        root: self.root.clone(),
      }
    }
  }
  impl<M: Monoid> Sequence<M> {
    pub fn new() -> Self {
      Self { root: None }
    }

    pub fn len(&self) -> usize {
      self.root.as_ref().map(|root| root.len()).unwrap_or(0)
    }

    pub fn split(mut self, at: usize) -> (Self, Self) {
      let mut after = Self::new();
      if let Some(root) = self.root.take() {
        let (x, y) = root.split(at);
        self.root = x;
        after.root = y;
      }
      (self, after)
    }

    pub fn append(&mut self, mut after: Self) {
      let root = Node::merge(self.root.take(), after.root.take());
      self.root = root;
    }

    pub fn push_back(&mut self, value: M::S) {
      let node = Node::with_value(value);
      self.root = Node::merge(self.root.take(), Some(node));
    }

    pub fn push_front(&mut self, value: M::S) {
      let node = Node::with_value(value);
      self.root = Node::merge(Some(node), self.root.take());
    }

    pub fn pop_back(&mut self) -> Option<M::S> {
      if let Some(root) = self.root.take() {
        let at = root.len() - 1;
        let (before, after) = root.split(at);
        self.root = before;
        after.map(|node| node.value)
      } else {
        None
      }
    }

    pub fn pop_front(&mut self) -> Option<M::S> {
      if let Some(root) = self.root.take() {
        let (before, after) = root.split(1);
        self.root = after;
        before.map(|node| node.value)
      } else {
        None
      }
    }

    pub fn insert(&mut self, at: usize, value: M::S) {
      let node = Node::with_value(value);
      self.root = if let Some(root) = self.root.take() {
        let (x, y) = root.split(at);
        Node::merge(x, Node::merge(Some(node), y))
      } else {
        Some(node)
      };
    }

    pub fn remove(&mut self, index: usize) {
      if let Some(root) = self.root.take() {
        let (mut x, w) = root.split(index);
        if let Some(w) = w {
          let (_, z) = w.split(1);
          x = Node::merge(x, z);
        }
        self.root = x;
      }
    }

    pub fn get(&self, index: usize) -> &M::S {
      self.root.as_ref().unwrap().at(index).value()
    }

    pub fn set(&mut self, index: usize, value: M::S) {
      self.root.as_mut().unwrap().set_value(index, value);
    }

    pub fn prod(&self, range: impl std::ops::RangeBounds<usize>) -> M::S {
      use std::ops::Bound::*;
      if let Some(root) = &self.root {
        let l = match range.start_bound() {
          Included(&l) => l,
          Excluded(&l) => l + 1,
          Unbounded => 0,
        };
        let r = match range.end_bound() {
          Included(&r) => r + 1,
          Excluded(&r) => r,
          Unbounded => self.len(),
        };
        root.prod(l, r)
      } else {
        M::e()
      }
    }

    pub fn print(&self) where M::S: std::fmt::Debug {
      let mut nodes = vec![];
      if let Some(root) = &self.root {
        for i in 0 .. self.len() {
          nodes.push(root.at(i));
        }
      }
      let height = self.root.as_ref().map(|node| node.height()).unwrap_or(0);
      let mut strs = vec![vec![]; height];
      for node in nodes {
        let s = format!("({:?}, {:?})", node.value(), node.prod(0, node.len()));
        let n = s.len();
        let j = node.height();
        for i in 0 .. height {
          if i == height - j {
            strs[i].push(s.clone());
          } else {
            strs[i].push(" ".repeat(n));
          }
        }
      }
      for s in strs {
        println!("{}", s.join(""));
      }
    }
  }

  pub struct Node<M: Monoid> {
    children: [Option<Box<Node<M>>>; 2],
    height: usize,
    len: usize,
    value: M::S,
    prod: M::S,
  }
  impl<M: Monoid> Clone for Node<M> where M::S: Clone {
    fn clone(&self) -> Self {
      Self {
        children: self.children.clone(),
        height: self.height,
        len: self.len,
        value: self.value.clone(),
        prod: self.prod.clone(),
      }
    }
  }
  impl<M: Monoid> std::fmt::Debug for Node<M> where M::S: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_fmt(format_args!("Node[len={}, height={}, value={:?}, prod={:?}] (", self.len, self.height, &self.value, &self.prod))?;
      if let Some(left) = &self.children[0] {
        f.write_fmt(format_args!("{:?}", left))?;
      } else {
        f.write_str("Nil")?;
      }
      f.write_str(", ")?;
      if let Some(right) = &self.children[1] {
        f.write_fmt(format_args!("{:?}", right))?;
      } else {
        f.write_str("Nil")?;
      }
      f.write_str(")")?;
      Ok(())
    }
  }
  impl<M: Monoid> Node<M> {
    pub fn with_value(value: M::S) -> Box<Self> {
      Box::new(Self {
        children: [None, None],
        height: 1,
        len: 1,
        prod: M::op(&M::e(), &value),
        value,
      })
    }

    pub fn len(&self) -> usize {
      self.len
    }

    pub fn height(&self) -> usize {
      self.height
    }

    pub fn value(&self) -> &M::S {
      &self.value
    }

    pub fn prod(&self, mut l: usize, mut r: usize) -> M::S {
      let mut prod = M::e();
      if l >= r {
        return prod;
      }
      if l == 0 && self.len <= r {
        prod = M::op(&prod, &self.prod);
        return prod;
      }
      if let Some(left) = &self.children[0] {
        prod = M::op(&left.prod(l, r), &prod);
        l = 0;
        r -= left.len;
      }
      if l <= 0 && 0 < r {
        prod = M::op(&prod, &self.value);
        l = 0;
        r -= 1;
      }
      if let Some(right) = &self.children[1] {
        prod = M::op(&prod, &right.prod(l, r));
      }
      prod
    }

    pub fn child(&self, dir: usize) -> Option<&Self> {
      assert!(dir < 2);
      Some(self.children[dir].as_ref()?)
    }

    pub fn set_value(&mut self, index: usize, value: M::S) {
      self.bottom_up_mut(index, &mut |node| {
        node.update_mut();
      }, |node| {
        node.value = value;
        node.update_mut();
      });
    }

    pub fn at(&self, mut index: usize) -> &Self {
      assert!(index < self.len);
      if let Some(left) = &self.children[0] {
        if index < left.len {
          return left.at(index);
        }
        index -= left.len;
      }
      if index == 0 {
        return self;
      }
      self.children[1].as_ref().unwrap().at(index - 1)
    }

    pub fn bottom_up_mut(&mut self, mut at: usize, way: &mut impl FnMut(&mut Self), leaf: impl FnOnce(&mut Self)) {
      if let Some(left) = &mut self.children[0] {
        if at < left.len {
          left.bottom_up_mut(at, way, leaf);
          (way)(self);
          return;
        }
        at -= left.len;
      }
      if at == 0 {
        (leaf)(self);
        return;
      }
      at -= 1;
      if let Some(right) = &mut self.children[1] {
        if at < right.len {
          right.bottom_up_mut(at, way, leaf);
          (way)(self);
        }
      }
    }

    pub fn split(mut self: Box<Self>, mut at: usize) -> (Option<Box<Self>>, Option<Box<Self>>) {
      if let Some(left) = self.children[0].take() {
        if at <= left.len {
          let (x, y) = left.split(at);
          self.children[0] = y;
          return (x, Some(self.balance()));
        }
        at -= left.len;
        self.children[0] = Some(left);
      }
      if at == 1 {
        let right = self.children[1].take();
        return (Some(self.balance()), right);
      }
      at -= 1;
      if let Some(right) = self.children[1].take() {
        if at <= right.len {
          let (x, y) = right.split(at);
          self.children[1] = x;
          return (Some(self.balance()), y);
        }
      }
      (Some(self), None)
    }

    pub fn merge(before: Option<Box<Self>>, after: Option<Box<Self>>) -> Option<Box<Self>> {
      if let Some(mut node) = before {
        let right = node.children[1].take();
        node.children[1] = Node::merge(right, after);
        Some(node.balance())
      } else {
        after
      }
    }

    fn update_move(&mut self) {
      let mut height = 1;
      let mut len = 1;
      for child in &self.children {
        if let Some(child) = child {
          height = height.max(child.height + 1);
          len += child.len;
        }
      }
      self.height = height;
      self.len = len;
      self.update_mut();
    }

    fn update_mut(&mut self) {
      let mut prod = M::e();
      if let Some(left) = &self.children[0] {
        prod = M::op(&prod, &left.prod);
      }
      prod = M::op(&prod, &self.value);
      if let Some(right) = &self.children[1] {
        prod = M::op(&prod, &right.prod);
      }
      self.prod = prod;
    }

    fn rotate(mut self: Box<Self>, dir: usize) -> Box<Self> {
      assert!(dir < 2);
      if let Some(mut x) = self.children[dir ^ 1].take() {
        let c = x.children[dir].take();
        self.children[dir ^ 1] = c;
        self.update_move();
        x.children[dir] = Some(self);
        x.update_move();
        x
      } else {
        self
      }
    }

    fn balance(mut self: Box<Self>) -> Box<Self> {
      let mut h = [0, 0];
      for dir in 0 .. 2 {
        if let Some(child) = &self.children[dir] {
          h[dir] = child.height;
        }
      }
      self = if h[0] + 2 <= h[1] {
        self.rotate(0)
      } else if h[0] >= h[1] + 2 {
        self.rotate(1)
      } else {
        self
      };
      self.update_move();
      self
    }
  }
}
