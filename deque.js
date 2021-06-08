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
      this.head = this.tail.reverse();
      this.tail = [];
    }
    return this.head.pop();
  }
  
  pop_back() {
    if (!this.tail.length) {
      this.tail = this.head.reverse();
      this.head = [];
    }
    return this.tail.pop();
  }
}
