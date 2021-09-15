#![allow(dead_code, unused_imports, unused_macros, non_snake_case)]

fn main() {
}

type Int = i64;
const MOD: Int = 1_000_000_007;
const INF: Int = 1_000_000_000;
const YESNO: [&'static str; 2] = ["Yes", "No"];

fn read_line() -> String { let mut buf = String::new(); std::io::stdin().read_line(&mut buf).ok(); buf }
fn read_words() -> Vec<String> { gets().split_whitespace().map(str::to_string).collect() }

fn yes() { println!("{}", YESNO[0]); }
fn no() { println!("{}", YESNO[1]); }
fn yesno(c: bool) { println!("{}", if c { YESNO[0] } else { YESNO[1] }); }
