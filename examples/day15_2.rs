use std::{collections::HashSet, io::BufRead};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const MAX_WIDTH: usize = WIDTH - 1;
const MAX_HEIGHT: usize = HEIGHT - 1;

const fn i_to_xy<const WIDTH: usize>(i: usize) -> (usize, usize) {
    (x::<WIDTH>(i), y::<WIDTH>(i))
}

const fn x<const WIDTH: usize>(i: usize) -> usize {
    i % WIDTH
}

const fn y<const WIDTH: usize>(i: usize) -> usize {
    i / WIDTH
}

const fn xy_to_i<const WIDTH: usize>(x: usize, y: usize) -> usize {
    x + (WIDTH * y)
}

fn find_adjacents<const WIDTH: usize, const MAX_WIDTH: usize, const MAX_HEIGHT: usize>(
    i: usize,
) -> Vec<usize> {
    let (x, y) = i_to_xy::<WIDTH>(i);

    if x > 0 && x < MAX_WIDTH && y > 0 && y < MAX_HEIGHT {
        return vec![
            // horizontal and vertical
            xy_to_i::<WIDTH>(x + 1, y),
            xy_to_i::<WIDTH>(x - 1, y),
            xy_to_i::<WIDTH>(x, y + 1),
            xy_to_i::<WIDTH>(x, y - 1),
            // diagonals
            // xy_to_i(some_x + 1, y + 1),
            // xy_to_i(some_x + 1, y - 1),
            // xy_to_i(some_x - 1, y + 1),
            // xy_to_i(some_x - 1, y - 1),
        ];
    }
    // upper left
    // (0, 0) => {
    if x == 0 && y == 0 {
        return vec![
            xy_to_i::<WIDTH>(1, 0),
            xy_to_i::<WIDTH>(0, 1),
            // xy_to_i(1, 1)
        ];
    }
    // lower left
    // (0, MAX_HEIGHT) => {
    if x == 0 && y == MAX_HEIGHT {
        return vec![
            xy_to_i::<WIDTH>(1, MAX_HEIGHT),
            xy_to_i::<WIDTH>(0, MAX_HEIGHT - 1),
            // xy_to_i(1, MAX_HEIGHT - 1),
        ];
    }
    // upper right
    // (MAX_WIDTH, 0) => {
    if x == MAX_WIDTH && y == 0 {
        return vec![
            xy_to_i::<WIDTH>(MAX_WIDTH - 1, 0),
            xy_to_i::<WIDTH>(MAX_WIDTH, 1),
            // xy_to_i(MAX_WIDTH - 1, 1),
        ];
    }
    // lower right
    // (MAX_WIDTH, MAX_HEIGHT) => {
    if x == MAX_WIDTH && y == MAX_HEIGHT {
        return vec![
            xy_to_i::<WIDTH>(MAX_WIDTH - 1, MAX_HEIGHT),
            xy_to_i::<WIDTH>(MAX_WIDTH, MAX_HEIGHT - 1),
            // xy_to_i(MAX_WIDTH - 1, MAX_HEIGHT - 1),
        ];
    }
    // right column
    // (MAX_WIDTH, some_y) => {
    if x == MAX_WIDTH {
        return vec![
            xy_to_i::<WIDTH>(MAX_WIDTH, y + 1),
            xy_to_i::<WIDTH>(MAX_WIDTH, y - 1),
            xy_to_i::<WIDTH>(MAX_WIDTH - 1, y),
            // xy_to_i(MAX_WIDTH - 1, some_y + 1),
            // xy_to_i(MAX_WIDTH - 1, some_y - 1),
        ];
    }
    // bottom row
    // (some_x, MAX_HEIGHT) => {
    if y == MAX_HEIGHT {
        return vec![
            xy_to_i::<WIDTH>(x + 1, MAX_HEIGHT),
            xy_to_i::<WIDTH>(x - 1, MAX_HEIGHT),
            xy_to_i::<WIDTH>(x, MAX_HEIGHT - 1),
            // xy_to_i(some_x + 1, MAX_HEIGHT - 1),
            // xy_to_i(some_x - 1, MAX_HEIGHT - 1),
        ];
    }
    // left column
    // (0, some_y) => {
    if x == 0 {
        return vec![
            xy_to_i::<WIDTH>(1, y),
            xy_to_i::<WIDTH>(0, y - 1),
            xy_to_i::<WIDTH>(0, y + 1),
            // xy_to_i(1, some_y - 1),
            // xy_to_i(1, some_y + 1),
        ];
    }
    // top row
    // (some_x, 0) => {
    if y == 0 {
        return vec![
            xy_to_i::<WIDTH>(x + 1, 0),
            xy_to_i::<WIDTH>(x - 1, 0),
            xy_to_i::<WIDTH>(x, 1),
            // xy_to_i(some_x + 1, 1),
            // xy_to_i(some_x - 1, 1),
        ];
    }

    vec![]
}
//  1  function Dijkstra(Graph, source):
//  2
//  3      create vertex set Q
//  4
//  5      for each vertex v in Graph:
//  6          dist[v] ← INFINITY
//  7          prev[v] ← UNDEFINED
//  8          add v to Q
//  9      dist[source] ← 0
// 10
// 11      while Q is not empty:
// 12          u ← vertex in Q with min dist[u]
// 13
// 14          remove u from Q
// 15
// 16          for each neighbor v of u still in Q:
// 17              alt ← dist[u] + length(u, v)
// 18              if alt < dist[v]:
// 19                  dist[v] ← alt
// 20                  prev[v] ← u
// 21
// 22      return dist[], prev[]

fn dijkstra<
    const SIZE: usize,
    const WIDTH: usize,
    const MAX_WIDTH: usize,
    const MAX_HEIGHT: usize,
>(
    graph: &[u8],
    source: usize,
    target: usize,
) -> Vec<usize> {
    let mut q = HashSet::new();
    let mut dist = [usize::MAX; SIZE];
    let mut prev = [usize::MAX; SIZE];

    for (i, _r) in graph.iter().enumerate() {
        q.insert(i);
    }

    dist[source] = 0;

    while !q.is_empty() {
        let mut qmin = usize::MAX;
        let mut u: usize = 0;

        for i in q.iter() {
            if dist[*i] < qmin {
                qmin = dist[*i];
                u = *i;
            }
        }

        q.remove(&u);

        let neighbors = find_adjacents::<WIDTH, MAX_WIDTH, MAX_HEIGHT>(u);
        let neighbors = neighbors.iter().filter(|neighbor| q.contains(neighbor));

        for neighbor in neighbors {
            let alt = dist[u] + graph[*neighbor] as usize;
            if alt < dist[*neighbor] {
                dist[*neighbor] = alt;
                prev[*neighbor] = u;
            }
        }
    }

    let mut out = vec![target];
    let mut u = target;

    while let Some(p) = prev.get(u) {
        if *p != usize::MAX {
            out.push(*p);
        }
        u = *p;
    }

    out.reverse();
    out
}

fn main() {
    let input = std::fs::File::open("inputs/15.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());

    let mut initial_risks_tile = [0u8; 100 * 100];

    for (y, row) in file_lines.enumerate() {
        for (x, risk_byte) in row.bytes().enumerate() {
            let risk_byte = [risk_byte];
            let risk_str = unsafe { std::str::from_utf8_unchecked(&risk_byte) };
            let risk = risk_str.parse::<u8>().unwrap();
            initial_risks_tile[xy_to_i::<100>(x, y)] = risk;
        }
    }

    let mut big = Vec::with_capacity(WIDTH * HEIGHT);

    for y in 0..5 {
        for row in initial_risks_tile.chunks(100) {
            for x in 0..5 {
                for risk in row {
                    let mut adjusted_risk = *risk;
                    let times_to_add_one = x + y;
                    for _ in 0..times_to_add_one {
                        adjusted_risk += 1;
                        adjusted_risk = if adjusted_risk > 9 { 1 } else { adjusted_risk };
                    }

                    big.push(adjusted_risk);
                }
            }
        }
    }

    let out =
        dijkstra::<{ WIDTH * HEIGHT }, WIDTH, MAX_WIDTH, MAX_HEIGHT>(&big, 0, WIDTH * HEIGHT - 1);

    let mut scores = vec![];
    for i in &out {
        scores.push(big[*i]);
    }

    dbg!(&scores[1..].iter().map(|n| *n as usize).sum::<usize>());
}
