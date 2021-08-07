#![allow(non_snake_case, dead_code, unused_imports, unused_macros)]

use std::cmp::Reverse;

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

const INF: usize = 1_000_000_000_000_000_000;

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
    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
        for j in 0..n {
            seen[i][j] = input.c[i][j] == '#';
        }
    }
    let mut all_seen = false;
    dfs(
        &input,
        input.s,
        &mut visited,
        &mut route,
        &mut seen,
        &mut all_seen,
    );
    let answer = route.iter().map(|idx| DIR[*idx]).collect::<String>();
    println!("{}", answer);
}

fn dfs(
    input: &Input,
    v: (usize, usize),
    visited: &mut [Vec<bool>],
    route: &mut Vec<usize>,
    seen: &mut [Vec<bool>],
    all_seen: &mut bool,
) {
    if *all_seen {
        return;
    }
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
    if !*all_seen && seen.iter().all(|row| row.iter().all(|cell| *cell)) {
        *all_seen = true;
        // 帰りの最短経路を考える
        let kaeri_michi = dijkstra(v.0, v.1, input);
        for r in kaeri_michi {
            route.push(r);
        }
        return;
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
            dfs(input, (next_i, next_j), visited, route, seen, all_seen);
            if *all_seen {
                return;
            }
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

// 帰り道用のダイクストラ
fn dijkstra(sh: usize, sw: usize, input: &Input) -> Vec<usize> {
    let mut dist = vec![vec![INF; input.n]; input.n];
    let mut prev = vec![vec![(INF, INF); input.n]; input.n];
    dist[sh][sw] = 0;
    let mut heap = std::collections::BinaryHeap::new();
    heap.push((Reverse(0), sh, sw));
    while !heap.is_empty() {
        let (now_cost, vh, vw) = heap.pop().unwrap();
        if vh == input.s.0 && vw == input.s.1 {
            break;
        }
        let c = now_cost.0;
        for (di, dj) in DIJ.iter() {
            let next_i = vh + di;
            let next_j = vw + dj;
            if next_i >= input.n || next_j >= input.n {
                continue;
            }
            if input.c[next_i][next_j] == '#' {
                continue;
            }
            if dist[next_i][next_j] <= (input.c[next_i][next_j] as i32 - 48) as usize + c {
                continue;
            }
            dist[next_i][next_j] = (input.c[next_i][next_j] as i32 - 48) as usize + c;
            prev[next_i][next_j] = (vh, vw);
            heap.push((Reverse(dist[next_i][next_j]), next_i, next_j));
        }
    }
    let mut h = input.s.0;
    let mut w = input.s.1;
    let mut route = vec![];
    while h != sh || w != sw {
        let (preh, prew) = prev[h][w];
        let r: usize = if preh < h {
            3
        } else if preh > h {
            0
        } else if prew < w {
            2
        } else {
            1
        };
        route.push(r);
        h = preh;
        w = prew;
    }
    route.reverse();
    route
}
