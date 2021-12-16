fn main() {
  let mut tree1 = AVLTree::new(1);
  let mut tree2 = AVLTree::new(2);
  let mut tree3 = AVLTree::new(3);
  let mut tree4 = AVLTree::new(4);
  tree3.insert(0, &mut tree1);
  tree1.insert(1, &mut tree2);
  tree3.insert(1, &mut tree4);
  for node in tree1.iter() {
    eprintln!("{:?}", node);
  }
}

use std::ptr::NonNull;
#[derive(Debug, Clone)]
pub struct AVLTree<T> {
  parent: Option<NonNull<AVLTree<T>>>,
  children: [Option<NonNull<AVLTree<T>>>; 2],
  len: usize, height: usize,
  value: T,
}
impl<T> AVLTree<T> {
  pub fn new(value: T) -> Self {
    Self {
      parent: None,
      children: [None, None],
      len: 1, height: 1,
      value,
    }
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn get(&self) -> &T {
    &self.value
  }

  pub fn get_mut(&mut self) -> &mut T {
    &mut self.value
  }

  pub fn child(&self, dir: usize) -> Option<&Self> {
    assert!(dir < 2);
    self.children[dir].map(|p| unsafe { &*p.as_ptr() })
  }

  pub fn child_mut(&self, dir: usize) -> Option<&mut Self> {
    assert!(dir < 2);
    self.children[dir].map(|p| unsafe { &mut *p.as_ptr() })
  }

  pub fn parent(&self) -> Option<&Self> {
    self.parent.map(|p| unsafe { &*p.as_ptr() })
  }

  pub fn parent_mut(&mut self) -> Option<&mut Self> {
    self.parent.map(|p| unsafe { &mut *p.as_ptr() })
  }

  pub fn at(&self, mut index: usize) -> Option<&Self> {
    if let Some(left) = self.child(0) {
      if index < left.len {
        return left.at(index);
      }
      index -= left.len;
    }
    if index == 0 {
      return Some(self);
    }
    if let Some(right) = self.child(1) {
      if index < right.len {
        return right.at(index);
      }
    }
    None
  }

  pub fn at_mut(&mut self, mut index: usize) -> Option<&mut Self> {
    let left_len = self.child(0).map(|c| c.len).unwrap_or(0);
    if index < left_len {
      return self.child_mut(0).unwrap().at_mut(index);
    }
    index -= left_len;
    if index == 0 {
      return Some(self);
    }
    if let Some(right) = self.child_mut(1) {
      if index < right.len {
        return right.at_mut(index);
      }
    }
    None
  }

  pub fn iter(&self) -> AVLTreeIter<'_, T> {
    AVLTreeIter(Some(self))
  }

  /// 左端または右端の要素を取得する
  pub fn most(&self, dir: usize) -> &Self {
    assert!(dir < 2);
    self.child(dir).map(|c| c.most(dir)).unwrap_or(self)
  }

  pub fn most_mut(&mut self, dir: usize) -> &mut Self {
    assert!(dir < 2);
    if self.children[dir].is_some() {
      self.child_mut(dir).unwrap().most_mut(dir)
    } else {
      self
    }
  }

  /// predicate が true を返すもっとも左またはもっとも右のノードを返す
  pub fn find_most(&self, dir: usize, mut predicate: impl FnMut(&Self) -> bool) -> Option<&Self> {
    if (predicate)(self) {
      self.child(dir).and_then(|c| c.find_most(dir, predicate)).or(Some(self))
    } else {
      self.child(dir ^ 1).and_then(|c| c.find_most(dir, predicate))
    }
  }

  /// 隣に挿入する
  pub fn insert(&mut self, dir: usize, node: &mut Self) {
    node.most_mut(dir).children[dir] = self.children[dir].take();
    self.children[dir] = Some(node.into());
    node.parent = Some(self.into());
    self.balance();
  }

  /// 隣の要素を取得
  /// O(logN) amortized
  pub fn sibling(&self, dir: usize) -> Option<&Self> {
    if let Some(child) = self.child(dir) {
      return Some(child.most(dir ^ 1));
    }
    let mut node = self;
    while let Some(parent) = node.parent() {
      if parent.dir(node as *const _).unwrap() == dir ^ 1 {
        return Some(parent);
      } else {
        node = parent;
      }
    }
    None
  }

  /// 隣の要素を取得
  /// O(logN) amortized
  pub fn sibling_mut(&mut self, dir: usize) -> Option<&mut Self> {
    if self.children[dir].is_some() {
      return Some(self.child_mut(dir)?.most_mut(dir ^ 1));
    }
    let mut node = self;
    while let Some(parent) = node.parent() {
      if parent.dir(node as *const _).unwrap() == dir ^ 1 {
        return node.parent_mut();
      } else {
        node = node.parent_mut()?;
      }
    }
    None
  }

  /// 平衡する
  pub fn balance(&mut self) {
    let hl = self.child(0).map(|c| c.height).unwrap_or(0);
    let hr = self.child(1).map(|c| c.height).unwrap_or(0);
    if hl > hr + 1 {
      let child = self.child_mut(0).unwrap();
      child.balance();
      child.rotate();
    } else if hl + 1 < hr {
      let child = self.child_mut(1).unwrap();
      child.balance();
      child.rotate();
    }
  }

  /// 自身を回転によって親との位置関係を入れ替える
  pub fn rotate(&mut self) {
    if let Some(parent) = self.parent.take().map(|p| unsafe { &mut *p.as_ptr() }) { // cut(p<-x)
      self.parent = parent.parent.replace(self.into()); // cut(a<-p), link(x<-p), link(a<-x)
      if let Some(ancestor) = self.parent.map(|p| unsafe { &mut *p.as_ptr() }) {
        let dir = ancestor.dir(parent as *const _).unwrap();
        ancestor.children[dir] = Some(self.into()); // link(a=>x)
      }
      let dir = parent.dir(self as *const _).unwrap();
      parent.children[dir] = self.children[dir ^ 1].replace(parent.into()); // cut(x=>c), link(x=>p), cut(p=>x), link(p=>c)
      if let Some(child) = parent.children[dir].map(|p| unsafe { &mut *p.as_ptr() }) {
        child.parent = Some(parent.into()); // cut(x<-c), link(p<-c)
      }
      parent.refresh();
      self.refresh();
    }
  }

  fn refresh(&mut self) {
    let mut len = 1;
    let mut height = 0;
    for dir in 0 .. 2 {
      if let Some(child) = self.child(dir) {
        len += child.len;
        height = height.max(child.height);
      }
    }
    self.len = len;
    self.height = height + 1;
  }

  fn dir(&self, child: *const AVLTree<T>) -> Option<usize> {
    (0 .. 2).find(|&dir| self.children[dir].map(|p| p.as_ptr() as *const _ == child).unwrap_or(false))
  }
}

pub struct AVLTreeIter<'a, T>(Option<&'a AVLTree<T>>);
impl<'a, T> Iterator for AVLTreeIter<'a, T> {
  type Item = &'a AVLTree<T>;
  fn next(&mut self) -> Option<Self::Item> {
    let next = self.0?;
    self.0 = next.sibling(1);
    Some(next)
  }
}
