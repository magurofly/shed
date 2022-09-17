function *main() {
  // (yield) で 1 行読み込み、 (yield n) で複数行読み込み
}

const iter = main();
let multiple = iter.next().value || 0, reading = [];
require("readline").createInterface({input: process.stdin}).on("line", line => {
  if (multiple <= 0) multiple = iter.next(line).value || 0;
  else if (multiple > 0) {
    reading.push(line);
    if (--multiple <= 0) multiple = iter.next(reading).value || 0, reading = [];
  }
});

function array(line) { return line.split(" "); }
function array_number(line) { return line.split(" ").map(Number); }
function array_bigint(line) { return line.split(" ").map(BigInt); }
function max(...xs) { let y = xs[0]; for (const x of xs) if (y < x) y = x; return y; }
function min(...xs) { let y = xs[0]; for (const x of xs) if (y > x) y = x; return y; }
function print(x) { console.log(x instanceof Array ? x.join(" ") : x + ""); }
function *range(l, r, step = null) {
  if (step == null) for (let i = l; i < r; i++) yield i;
  else if (step >= 0) for (let i = l; i < r; i += step) yield i;
  else for (let i = r + step; i >= l; i += step) yield i;
}
Object.defineProperties(Array.prototype, {
  first: { get() { return this[0]; }, set(x) { this[0] = x; } },
  last: { get() { return this[this.length - 1]; }, set(x) { this[this.length - 1] = x; } },
  swap: { value(i, j) { const x = this[i]; this[i] = this[j]; this[i] = x; } },
});

class Deque {
  constructor(b = []) { this.a = []; this.b = b.slice(); }
  get length() { return this.a.length + this.b.length; }
  get(i) { return (i < this.a.length) ? this.a[i] : this.b[this.b.length - 1 - (i - this.a.length)]; }
  set(i, x) { if (i < this.a.length) this.a[i] = x; else this.b[this.b.length - 1 - (i - this.a.length)] = x; }
  push_front(...xs) { this.a.push(...xs); }
  push_back(...xs) { this.b.push(...xs); }
  pop_front() { if (!this.a.length) this.a = this.b.splice(0, this.b.length + 1 >> 1).reverse(); return this.a.pop(); }
  pop_back() { if (this.b.length == 0) this.b = this.a.splice(0, this.a.length + 1 >> 1).reverse(); return this.b.pop(); }
  *[Symbol.iterator]() { for (const x of this.a) yield x; for (let i = this.b.length; i--; ) yield this.b[i]; }
}
