#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use proconio::{input, marker::*, source::Source};

// --releaseでoverflow無視
// DIJ と DIRが合っているか確認する
const DIJ: [(usize, usize); 4] = [
    (1usize.wrapping_neg(), 0),
    (0, 1usize.wrapping_neg()),
    (0, 1),
    (1, 0),
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
        s: (usize, usize),
        c: [Chars; n],
    }
    let input = Input { n, s, c };
    let mut route = vec![];
    let mut visited = vec![vec![false; n]; n];
    dfs_iki(&input, input.s, &mut visited, &mut route);
    let answer_iki = route.iter().map(|idx| DIR[*idx]).collect::<String>();
    route.reverse();
    for r in route.iter_mut() {
        *r = match *r {
            0 => 3,
            1 => 2,
            2 => 1,
            3 => 0,
            _ => unreachable!(),
        };
    }
    let answer_kaeri = route.iter().map(|idx| DIR[*idx]).collect::<String>();
    println!("{}{}", answer_iki, answer_kaeri);
}

fn dfs_iki(input: &Input, v: (usize, usize), visited: &mut [Vec<bool>], route: &mut Vec<usize>) {
    for (oi, (di, dj)) in DIJ.iter().enumerate() {
        let next_i = v.0 + *di;
        let next_j = v.1 + *dj;
        if next_i >= input.n
            || next_j >= input.n
            || visited[next_i][next_j]
            || input.c[next_i][next_j] == '#'
        {
            continue;
        }
        visited[next_i][next_j] = true;
        route.push(oi);
        dfs_iki(input, (next_i, next_j), visited, route);
        return;
    }
}
