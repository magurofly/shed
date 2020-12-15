#![allow(dead_code, unused_imports)]

#[fastout]
fn main() {
  input! {
  }
  
}

const YESNO: [&'static str; 2] = ["Yes", "No"];

use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap};
use std::cmp::{self, Reverse}; // cmp::max, cmp::min
use itertools::Itertools as _;

fn yes() { println!("{}", YESNO[0]); }
fn no() { println!("{}", YESNO[1]); }
fn yesno(c: bool) { println!("{}", if c { YESNO[0] } else { YESNO[1] }); }
