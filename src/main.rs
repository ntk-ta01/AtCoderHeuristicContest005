#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::{input, marker::*, source::Source};

const DIJ: [(usize, usize); 4] = [
    (0, 1usize.wrapping_neg()),
    (1, 0),
    (0, 1),
    (1usize.wrapping_neg(), 0),
];
const DIR: [char; 4] = ['U', 'L', 'R', 'D'];

type Output = String;

struct Input {
    n: usize,
    s: (usize, usize),
    c: Vec<Vec<char>>,
}

fn main() {
    input! {
        n: usize,
        _: (usize, usize),
        _: [Chars; n],
    }
    println!();
}
