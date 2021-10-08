use std::cmp::{*, Ordering::*};
use std::boxed::*;

#[derive(Debug, Clone)]
/// AVL木
/// AVL木は、どの頂点についても、その左右の部分木の高さがたかだか1であるような木。
/// この特性の実現のために、すべての頂点は高さ `height` を持つ。
pub struct AVLTree<K, V> {
  root: Option<Box<AVLTreeNode<K, V>>>,
}

impl<K: Ord, V> AVLTree<K, V> {
  pub fn new() -> Self {
    Self { root: None }
  }

  pub fn insert(&mut self, key: K, value: V) where K: Ord {
    let node = AVLTreeNode::new(key, value);
    let root = if let Some(root) = self.root.take() { root.insert(node) } else { node };
    self.root.insert(root);
  }

  pub fn remove(&mut self, key: &K) where K: Ord {
    if let Some(root) = self.root.take() {
      self.root = root.remove(key);
    }
  }

  pub fn get(&self, key: &K) -> Option<&V> where K: Ord {
    Some(&self.root.as_ref()?.search(key)?.value)
  }

  pub fn get_mut(&mut self, key: &K) -> Option<&mut V> where K: Ord {
    Some(&mut self.root.as_mut()?.search_mut(key)?.value)
  }
}


#[derive(Debug, Clone)]
pub struct AVLTreeNode<K, V> {
  key: K,
  value: V,
  height: usize,
  children: [Option<Box<AVLTreeNode<K, V>>>; 2],
}

impl<K: Ord, V> AVLTreeNode<K, V> {
  pub fn new(key: K, value: V) -> Box<Self> {
    Box::new(Self {
      key,
      value,
      height: 1,
      children: [None, None],
    })
  }

  pub fn search<'a>(self: &'a Box<Self>, key: &K) -> Option<&'a Box<Self>> {
    match self.key.cmp(key) {
      Equal => Some(self),
      Greater => self.children[L].as_ref().and_then(|child| child.search(key)),
      Less => self.children[R].as_ref().and_then(|child| child.search(key)),
    }
  }

  pub fn search_mut<'a>(self: &'a mut Box<Self>, key: &K) -> Option<&'a mut Box<Self>> {
    match self.key.cmp(key) {
      Equal => Some(self),
      Greater => self.children[L].as_mut().and_then(|child| child.search_mut(key)),
      Less => self.children[R].as_mut().and_then(|child| child.search_mut(key)),
    }
  }

  pub fn insert(mut self: Box<Self>, mut node: Box<Self>) -> Box<Self> {
    match self.key.cmp(&node.key) {
      Equal => {
        node.children = self.children;
        node.refresh();
        node
      }
      Greater => {
        let child = if let Some(child) = self.children[0].take() { child.insert(node) } else { node };
        self.children[L].insert(child);
        self.refresh();
        self.balance()
      }
      Less => {
        let child = if let Some(child) = self.children[1].take() { child.insert(node) } else { node };
        self.children[R].insert(child);
        self.refresh();
        self.balance()
      }
    }
  }

  pub fn remove(mut self: Box<Self>, key: &K) -> Option<Box<Self>> {
    match self.key.cmp(key) {
      Equal => {
        let [left, right] = self.children;
        if let Some(l) = left {
          if let Some(r) = right {
            Some(l.insert(r))
          } else {
            Some(l)
          }
        } else {
          right
        }
      }
      Greater => {
        self.children[L] = self.children[L].take().and_then(|child| child.remove(key));
        self.refresh();
        Some(self.balance())
      }
      Less => {
        self.children[R] = self.children[R].take().and_then(|child| child.remove(key));
        self.refresh();
        Some(self.balance())
      }
    }
  }

  fn balanced(self: &Box<Self>) -> BalanceProperty {
    let hl = Self::height(&self.children[L]);
    let hr = Self::height(&self.children[R]);
    if hl >= hr + 2 {
      Left
    } else if hl + 2 <= hr {
      Right
    } else {
      Balanced
    }
  }

  /// 平衡を保つ
  fn balance(self: Box<Self>) -> Box<Self> {
    match self.balanced() {
      Balanced => self,
      Left => {
        match self.children[L].as_ref().unwrap().balanced() {
          Right => self.rotate2(TO_R),
          _ => self.rotate(TO_R),
        }
      }
      Right => {
        match self.children[R].as_ref().unwrap().balanced() {
          Left => self.rotate2(TO_L),
          _ => self.rotate(TO_L),
        }
      }
    }
  }

  /// `dir` 方向に単回転する
  fn rotate(mut self: Box<Self>, dir: usize) -> Box<Self> {
    assert!(dir < 2);
    let mut child = self.children[dir].take().unwrap();
    self.children[dir] = child.children[1 ^ dir].take();
    self.refresh();
    child.children[1 ^ dir].insert(self);
    child.refresh();
    child
  }

  /// `dir` 方向に二重回転する
  fn rotate2(mut self: Box<Self>, dir: usize) -> Box<Self> {
    assert!(dir < 2);
    let mut child = self.children[dir].take().unwrap();
    let mut decendant = child.children[1 ^ dir].take().unwrap();
    self.children[dir] = decendant.children[1 ^ dir].take();
    self.refresh();
    child.children[1 ^ dir] = decendant.children[dir].take();
    child.refresh();
    decendant.children[1 ^ dir].insert(self);
    decendant.children[dir].insert(child);
    decendant.refresh();
    decendant
  }

  fn height(node: &Option<Box<Self>>) -> usize {
    node.as_ref().map(|this| this.height).unwrap_or(0)
  }

  /// 状態を更新する
  fn refresh(&mut self) {
    self.height = Self::height(&self.children[L]).max(Self::height(&self.children[R])) + 1;
  }
}

const L: usize = 0;
const R: usize = 1;
const TO_R: usize = 0;
const TO_L: usize = 1;

enum BalanceProperty {
  Balanced,
  Left,
  Right,
}
use BalanceProperty::*;
