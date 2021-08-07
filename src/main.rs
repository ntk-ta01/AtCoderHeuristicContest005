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
    let mut seen = vec![vec![false; n]; n];
    dfs(&input, input.s, &mut visited, &mut route, &mut seen);
    let answer = route.iter().map(|idx| DIR[*idx]).collect::<String>();
    println!("{}", answer);
}

fn dfs(
    input: &Input,
    v: (usize, usize),
    visited: &mut [Vec<bool>],
    route: &mut Vec<usize>,
    seen: &mut [Vec<bool>],
) {
    // 頂点vから視界に入るところを全部trueに
    for (di, dj) in DIJ.iter() {
        let mut next_i = v.0 + *di;
        let mut next_j = v.1 + *dj;
        while next_i < input.n && next_j < input.n && input.c[next_i][next_j] != '#' {
            seen[next_i][next_j] = true;
            next_i += *di;
            next_j += *dj;
        }
    }
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
        if !is_partolled(input, next_i, next_j, (*di, *dj), seen) {
            dfs(input, (next_i, next_j), visited, route, seen);
        }
        let b_oi = match oi {
            0 => 3,
            1 => 2,
            2 => 1,
            3 => 0,
            _ => unreachable!(),
        };
        route.push(b_oi);
    }
}

// そのまま進んだ場合に周辺の道がすべてパトロールされているかチェック
fn is_partolled(
    input: &Input,
    next_i: usize,
    next_j: usize,
    dir: (usize, usize),
    seen: &[Vec<bool>],
) -> bool {
    let mut next_i = next_i;
    let mut next_j = next_j;
    let (nextnext_i1, nextnext_j1, nextnext_i2, nextnext_j2) = if dir.0 == 0 {
        // 横向きに直進 上と下を調べる
        if next_i == 0 {
            (0, 0, 1, 0)
        } else if next_i == input.n - 1 {
            (1usize.wrapping_neg(), 0, 0, 0)
        } else {
            (1usize.wrapping_neg(), 0, 1, 0)
        }
    } else {
        // 縦向きに直進 左と右を調べる
        if next_j == 0 {
            (0, 0, 0, 1)
        } else if next_j == input.n - 1 {
            (0, 1usize.wrapping_neg(), 0, 0)
        } else {
            (0, 1usize.wrapping_neg(), 0, 1)
        }
    };
    while next_i < input.n && next_j < input.n && input.c[next_i][next_j] != '#' {
        // next_i, next_jの周辺が全てseen || '#'ならOK
        if !(seen[next_i + nextnext_i1][next_j + nextnext_j1]
            || input.c[next_i + nextnext_i1][next_j + nextnext_j1] == '#')
            || !(seen[next_i + nextnext_i2][next_j + nextnext_j2]
                || input.c[next_i + nextnext_i2][next_j + nextnext_j2] == '#')
        {
            return false;
        }
        next_i += dir.0;
        next_j += dir.1;
    }
    true
}
