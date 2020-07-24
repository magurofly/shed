class Mint {
	static mod(a, p) { let r = a % p; while (r < 0) r += p; return r; }
	static pow(a, e, p) { let r = 1; while (e > 0) { if ((e & 1) == 1) r = r * a % p; a = a * a % p; } return r; }
	static inv(a, p) { return Mint.pow(a, p-2, p); }
	static factorials(n, p) { let f = [1], inv = []; for (let i = 1; i <= n; i++) inv[i] = Mint.inv(f[i] = i * f[i-1] % p, p); return [f, inv]; }
	constructor(value, mod) { this.mod = mod; this.value = Mint.mod(value, mod); }
	valueOf() { return this.value; }
	add(x) { return new Mint(this.value + x, this.mod); }
	sub(x) { return new Mint(this.value - x, this.mod); }
	mul(x) { return new Mint(this.value * x, this.mod); }
	div(x) { return new Mint(this.value * Mint.inv(x, this.mod), this.mod); }
	inv() { return new Mint(Mint.inv(this.value, this.mod), this.mod); }
}
