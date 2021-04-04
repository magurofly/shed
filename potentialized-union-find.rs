// 参考: https://github.com/noshi91/Library/blob/master/data_structure/potentialized_union_find.cpp
fn main() {
    let mut uf = PotentializedUnionFind::new(3, |x, y| x + y, || 0, |x| -x);
    println!("find(1) = {}", uf.find(1));
    println!("is_same(1, 2) = {}", uf.is_same(1, 2));
    println!("size(2) = {}", uf.size(2));
    println!("potential(2) = {}", uf.potential(2));
    println!("union(1, 2, 3)"); uf.union(1, 2, 1);
    println!("potential(1) = {}", uf.potential(1));
    println!("potential(2) = {}", uf.potential(2));
    println!("union(0, 1, -1)"); uf.union(0, 1, 1);
    println!("potential(0) = {}", uf.potential(0));
    println!("potential(1) = {}", uf.potential(1));
    println!("potential(2) = {}", uf.potential(2));
    println!("distance(0, 2) = {}", uf.distance(0, 2));
    
}

// 重みつきUnionFind
#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    parent: usize,
    size: usize,
}

#[derive(Clone, Debug)]
struct PotentializedUnionFind<T, Op: Fn(T, T) -> T, Id: Fn() -> T, Inv: Fn(T) -> T> {
    operation: Op,
    identity: Id,
    inverse: Inv,
    tree: std::cell::RefCell<Vec<Node<T>>>,
}
impl<T: Clone, Op: Fn(T, T) -> T, Id: Fn() -> T, Inv: Fn(T) -> T> PotentializedUnionFind<T, Op, Id, Inv> {
    fn new(size: usize, operation: Op, identity: Id, inverse: Inv) -> Self {
        let mut tree = Vec::with_capacity(size);
        for i in 0 .. size {
            tree.push(Node {
                value: (identity)(),
                parent: i,
                size: 1,
            });
        }
        Self {
            operation,
            identity,
            inverse,
            tree: std::cell::RefCell::new(tree),
        }
    }

    fn compress(&self, v: usize) {
        let p = self.tree.borrow()[v].parent;
        if p == v { return; }
        self.compress(p);
        let mut tree = self.tree.borrow_mut();
        tree[v].value = (self.operation)(tree[p].value.clone(), tree[v].value.clone());
    }

    fn find(&self, v: usize) -> usize {
        self.compress(v);
        self.tree.borrow()[v].parent
    }

    fn is_same(&self, u: usize, v: usize) -> bool {
        self.compress(u);
        self.find(u) == self.find(v)
    }

    fn size(&self, v: usize) -> usize {
        self.tree.borrow()[self.find(v)].size
    }

    fn potential(&self, v: usize) -> T {
        self.compress(v);
        self.tree.borrow()[v].value.clone()
    }

    fn distance(&self, u: usize, v: usize) -> T {
        (self.operation)((self.inverse)(self.potential(u)), self.potential(v))
    }

    fn union(&mut self, mut u: usize, mut v: usize, mut d: T) {
        u = self.find(u);
        v = self.find(v);
        let mut tree = self.tree.borrow_mut();
        if tree[u].size < tree[v].size {
            d = (self.inverse)(d);
            std::mem::swap(&mut u, &mut v);
        }
        d = (self.operation)((self.operation)(tree[u].value.clone(), d), (self.inverse)(tree[v].value.clone()));
        tree[u].size += tree[v].size;
        tree[v].parent = u;
        tree[v].value = d;
    }
}
