class Deque {
  constructor() {
    this.head = [];
    this.tail = [];
  }
  
  push_front(x) {
    this.head.push(x);
  }
  
  push_back(x) {
    this.tail.push(x);
  }
  
  pop_front() {
    if (!this.head.length) {
      while (this.tail.length) this.head.push(this.tail.pop());
    }
    return this.head.pop();
  }
  
  pop_back() {
    if (!this.tail.length) {
      while (this.head.length) this.tail.push(this.head.pop());
    }
    return this.tail.pop();
  }
}
