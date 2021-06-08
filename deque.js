class Deque {
  constructor() {
    this.head = [];
    this.tail = [];
  }
  
  push_front(...xs) {
    this.head.push(...xs);
  }
  
  push_back(...xs) {
    this.tail.push(...xs);
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
