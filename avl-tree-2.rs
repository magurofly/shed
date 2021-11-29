fn main() {
    let mut tree = AVLTree::new();
    for &i in &[1, 2, 3, 4, 5, 6, 7] {
        tree.insert(i);
    }
    eprintln!("{:?}", &tree);
}

#[derive(Debug, Clone)]
pub struct AVLTree<K> {
    root: Option<Box<Node<K>>>,
    len: usize,
}

impl<K: Ord> AVLTree<K> {
    pub fn new() -> Self {
        Self {
            root: None,
            len: 0,
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn contains(&self, key: &K) -> bool {
        Node::find(&self.root, &key).is_some()
    }
    
    pub fn insert(&mut self, key: K) -> bool {
        if self.contains(&key) {
            return false;
        }
        
        self.root = Node::insert(self.root.take(), Node::new(key));
        self.len += 1;
        
        true
    }
    
    pub fn remove(&mut self, key: &K) -> bool {
        if self.contains(key) {
            self.root = Node::remove(self.root.take(), key);
            self.len -= 1;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node<K> {
    key: K,
    child: [Option<Box<Node<K>>>; 2],
    height: usize,
}

impl<K: Ord> Node<K> {
    pub fn new(key: K) -> Box<Self> {
        Box::new(Self { key, child: [None, None], height: 1 })
    }
    
    fn rotate(mut self: Box<Self>, dir: usize) -> Box<Self> {
        if let Some(mut child) = self.child[1 ^ dir].take() {
            self.child[1 ^ dir] = child.child[dir].take();
            self.eval();
            child.child[dir] = Some(self);
            child.eval();
            child
        } else {
            self
        }
    }
    
    fn eval(&mut self) {
        let mut height = 1;
        for i in 0 .. 2 {
            if let Some(child) = &self.child[i] {
                height = height.max(child.height + 1);
            }
        }
        self.height = height;
    }
    
    fn balance(mut self: Box<Self>) -> Box<Self> {
        let mut hs = [0, 0];
        for i in 0 .. 2 {
            if let Some(child) = &self.child[i] {
                hs[i] = hs[i].max(child.height + 1);
            }
        }
        for i in 0 .. 2 {
            if hs[i] + 2 <= hs[1 ^ i] {
                self = self.rotate(i);
            }
        }
        self.eval();
        self
    }
    
    pub fn insert(this: Option<Box<Self>>, child: Box<Self>) -> Option<Box<Self>> {
        use std::cmp::Ordering::*;
        if let Some(mut node) = this {
            match node.key.cmp(&child.key) {
                Greater => {
                    node.child[0] = Self::insert(node.child[0].take(), child);
                }
                Less => {
                    node.child[1] = Self::insert(node.child[1].take(), child);
                }
                Equal => {
                    node = child;
                }
            }
            Some(node.balance())
        } else {
            Some(child)
        }
    }
    
    pub fn remove(this: Option<Box<Self>>, key: &K) -> Option<Box<Self>> {
        use std::cmp::Ordering::*;
        if let Some(mut node) = this {
            match node.key.cmp(key) {
                Greater => {
                    node.child[0] = Self::remove(node.child[0].take(), key);
                }
                Less => {
                    node.child[1] = Self::remove(node.child[1].take(), key);
                }
                Equal => {
                    return None;
                }
            }
            Some(node.balance())
        } else {
            this
        }
    }
    
    fn find<'a>(this: &'a Option<Box<Self>>, key: &K) -> &'a Option<Box<Self>> {
        use std::cmp::Ordering::*;
        if this.as_ref().map(|node| node.key == *key).unwrap_or(true) {
            return this;
        }
        let node = this.as_ref().unwrap();
        match node.key.cmp(key) {
            Greater => {
                return Self::find(&node.child[0], key);
            }
            Less => {
                return Self::find(&node.child[1], key);
            }
            _ => unreachable!()
        }
    }
}
