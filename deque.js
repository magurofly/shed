// License: CC0 1.0 Universal
// https://www.slideshare.net/catupper/amortize-analysis-of-deque-with-2-stack
class Deque {
  constructor() {
    this.front = [];
    this.back = [];
  }

  get length() {
    return this.front.length + this.back.length;
  }
  
  get(index) {
    if (index < this.front.length) return this.front[index];
    return this.back[this.back.length - 1 - (index - this.front.length)];
  }
  
  push_front(...xs) {
    this.front.push(...xs);
  }
  
  push_back(...xs) {
    this.back.push(...xs);
  }
  
  pop_front() {
    if (this.front.length == 0) this.front = this.back.splice(0, this.back.length + 1 >> 1).reverse();
    return this.front.pop();
  }
  
  pop_back() {
    if (this.back.length == 0) this.back = this.front.splice(0, this.front.length + 1 >> 1).reverse();
    return this.back.pop();
  }
  
  *[Symbol.iterator]() {
    for (const value of this.front) yield value;
    for (let i = this.back.length; i--; ) yield this.back[i];
  }
}
