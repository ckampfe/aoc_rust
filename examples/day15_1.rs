use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::BufRead;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct IndexWithCost {
    cost: usize,
    i: usize,
}

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
    let mut q = BinaryHeap::new();
    let mut dist = [usize::MAX; SIZE];
    let mut prev = [usize::MAX; SIZE];

    let index_with_cost = Reverse(IndexWithCost {
        cost: dist[source],
        i: source,
    });

    q.push(index_with_cost);

    dist[source] = 0;

    while let Some(u) = q.pop() {
        let Reverse(IndexWithCost { i: u, .. }) = u;

        let neighbors = find_adjacents::<WIDTH, MAX_WIDTH, MAX_HEIGHT>(u);

        for neighbor in neighbors {
            let alt = dist[u] + graph[neighbor] as usize;
            if alt < dist[neighbor] {
                dist[neighbor] = alt;
                prev[neighbor] = u;
                let neighbor_with_cost = Reverse(IndexWithCost {
                    cost: dist[neighbor],
                    i: neighbor,
                });
                q.push(neighbor_with_cost);
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

    let mut risks = [0u8; WIDTH * HEIGHT];

    for (y, row) in file_lines.enumerate() {
        for (x, risk_byte) in row.bytes().enumerate() {
            let risk_byte = [risk_byte];
            let risk_str = unsafe { std::str::from_utf8_unchecked(&risk_byte) };
            let risk = risk_str.parse::<u8>().unwrap();
            risks[xy_to_i::<WIDTH>(x, y)] = risk;
        }
    }

    let out =
        dijkstra::<{ WIDTH * HEIGHT }, WIDTH, MAX_WIDTH, MAX_HEIGHT>(&risks, 0, WIDTH * HEIGHT - 1);

    let mut scores = vec![];
    for i in &out {
        scores.push(risks[*i]);
    }

    dbg!(&scores[1..].iter().map(|n| *n as usize).sum::<usize>());
}

#[cfg(test)]
mod tests {

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const MAX_WIDTH: usize = WIDTH - 1;
    const MAX_HEIGHT: usize = HEIGHT - 1;

    #[test]
    fn small_case() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        let mut risks = [0u8; 10 * 10];

        let lines = input.split("\n");

        for (y, row) in lines.enumerate() {
            for (x, risk_byte) in row.bytes().enumerate() {
                let risk_byte = [risk_byte];
                let risk_str = unsafe { std::str::from_utf8_unchecked(&risk_byte) };
                let risk = risk_str.parse::<u8>().unwrap();
                risks[super::xy_to_i::<WIDTH>(x, y)] = risk;
            }
        }

        let out = super::dijkstra::<{ WIDTH * HEIGHT }, WIDTH, MAX_WIDTH, MAX_HEIGHT>(
            &risks,
            0,
            WIDTH * HEIGHT - 1,
        );

        let mut scores = vec![];
        for i in &out {
            scores.push(risks[*i]);
        }

        dbg!(out);
        dbg!(scores);

        assert!(false)
    }
}
