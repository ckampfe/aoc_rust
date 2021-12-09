use std::collections::{BTreeSet, VecDeque};
use std::io::BufRead;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const MAX_WIDTH: usize = WIDTH - 1;
const MAX_HEIGHT: usize = HEIGHT - 1;
const MAX_INDEX: usize = WIDTH * HEIGHT - 1;

struct HeightMap(Vec<u8>);

impl HeightMap {
    fn push(&mut self, location: u8) {
        self.0.push(location);
    }
}

fn find_adjacents(i: usize, a: &mut [usize; 4]) {
    *a = [usize::MAX; 4];

    let (x, y) = i_to_xy(i);

    match (x, y) {
        // regular case
        (some_x, some_y)
            if some_x > 0 && some_x < MAX_WIDTH && some_y > 0 && some_y < MAX_HEIGHT =>
        {
            a[0] = xy_to_i(some_x + 1, y);
            a[1] = xy_to_i(some_x - 1, y);
            a[2] = xy_to_i(some_x, y + 1);
            a[3] = xy_to_i(some_x, y - 1);
        }
        // upper left
        (0, 0) => {
            a[0] = xy_to_i(1, 0);
            a[1] = xy_to_i(0, 1);
        }
        // lower left
        (0, MAX_HEIGHT) => {
            a[0] = xy_to_i(1, MAX_HEIGHT);
            a[1] = xy_to_i(0, MAX_HEIGHT - 1);
        }
        // upper right
        (MAX_WIDTH, 0) => {
            a[0] = xy_to_i(MAX_WIDTH - 1, 0);
            a[1] = xy_to_i(MAX_WIDTH, 1);
        }
        // lower right
        (MAX_WIDTH, MAX_HEIGHT) => {
            a[0] = xy_to_i(MAX_WIDTH - 1, MAX_HEIGHT);
            a[1] = xy_to_i(MAX_WIDTH, MAX_HEIGHT - 1);
        }
        // right column
        (MAX_WIDTH, some_y) => {
            a[0] = xy_to_i(MAX_WIDTH, some_y + 1);
            a[1] = xy_to_i(MAX_WIDTH, some_y - 1);
            a[2] = xy_to_i(MAX_WIDTH - 1, some_y);
        }
        // bottom row
        (some_x, MAX_HEIGHT) => {
            a[0] = xy_to_i(some_x + 1, MAX_HEIGHT);
            a[1] = xy_to_i(some_x - 1, MAX_HEIGHT);
            a[2] = xy_to_i(some_x, MAX_HEIGHT - 1);
        }
        // left column
        (0, some_y) => {
            a[0] = xy_to_i(1, some_y);
            a[1] = xy_to_i(0, some_y - 1);
            a[2] = xy_to_i(0, some_y + 1);
        }
        // top row
        (some_x, 0) => {
            a[0] = xy_to_i(some_x + 1, 0);
            a[1] = xy_to_i(some_x - 1, 0);
            a[2] = xy_to_i(some_x, 1);
        }
        _ => unreachable!("unhandled index case"),
    };
}

const fn i_to_xy(i: usize) -> (usize, usize) {
    (x(i), y(i))
}

const fn x(i: usize) -> usize {
    i % WIDTH
}

const fn y(i: usize) -> usize {
    i / WIDTH
}

const fn xy_to_i(x: usize, y: usize) -> usize {
    x + (WIDTH * y)
}

fn is_lowpoint(value: u8, adjacent_values: &[u8]) -> bool {
    adjacent_values
        .iter()
        .all(|adjacent_value| value < *adjacent_value)
}

fn find_basin_indexes(
    i: usize,
    heightmap: &HeightMap,
    queue: &mut VecDeque<usize>,
    basin_buf: &mut Vec<usize>,
    adjacents: &mut [usize; 4],
) {
    queue.clear();

    basin_buf.clear();

    queue.push_back(i);

    basin_buf.push(i);

    while let Some(this_i) = queue.pop_front() {
        find_adjacents(this_i, adjacents);

        for adjacent in adjacents.iter() {
            if *adjacent <= MAX_INDEX && !basin_buf.contains(adjacent) && heightmap.0[*adjacent] < 9
            {
                basin_buf.push(*adjacent);
                queue.push_back(*adjacent);
            }
        }
    }
}

fn main() {
    let input = std::fs::File::open("inputs/9.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());

    let mut heightmap: HeightMap = HeightMap(Vec::with_capacity(WIDTH * HEIGHT));

    for line in file_lines {
        for value_at_location in line.split("").filter(|s| !s.is_empty()) {
            let value_at_location = value_at_location.parse::<u8>().unwrap();
            heightmap.push(value_at_location);
        }
    }

    assert!(heightmap.0.len() == WIDTH * HEIGHT);

    let mut basins_indexes: BTreeSet<usize> = BTreeSet::new();

    let mut basin_lengths: BTreeSet<usize> = BTreeSet::new();

    let mut queue: VecDeque<usize> = VecDeque::new();

    let mut basin_buf: Vec<usize> = Vec::new();

    let mut adjacents_buf = [usize::MAX; 4];

    for (i, value) in heightmap.0.iter().enumerate() {
        if basins_indexes.contains(&i) {
            continue;
        }

        find_adjacents(i, &mut adjacents_buf);

        let adjacent_values: Vec<u8> = adjacents_buf
            .into_iter()
            .filter(|adjacent_i| *adjacent_i <= MAX_INDEX)
            .map(|adjacent_i| heightmap.0[adjacent_i])
            .collect();

        if is_lowpoint(*value, &adjacent_values) {
            find_basin_indexes(
                i,
                &heightmap,
                &mut queue,
                &mut basin_buf,
                &mut adjacents_buf,
            );
            basin_lengths.insert(basin_buf.len());
            let basin_iter = basin_buf.iter();
            basins_indexes.extend(basin_iter);
        }
    }

    dbg!(basin_lengths.len());
    let top_3 = &basin_lengths.iter().rev().take(3).collect::<Vec<_>>();
    dbg!(top_3);
    dbg!(top_3[0] * top_3[1] * top_3[2]);
}
