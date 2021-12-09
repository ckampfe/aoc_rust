use std::{collections::VecDeque, io::BufRead};

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
        _ => panic!("unhandled index case"),
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

fn find_basin_indexes(i: usize, heightmap: &HeightMap, adjacents: &mut [usize; 4]) -> Vec<usize> {
    let mut basin = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(i);
    basin.push(i);

    while let Some(this_i) = queue.pop_front() {
        find_adjacents(this_i, adjacents);

        for adjacent in adjacents.iter() {
            if *adjacent <= MAX_INDEX && !basin.contains(adjacent) && heightmap.0[*adjacent] < 9 {
                basin.push(*adjacent);
                queue.push_back(*adjacent);
            }
        }
    }

    basin
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

    let mut basin_lens = vec![];

    let mut adjacents = [usize::MAX; 4];

    for (i, value) in heightmap.0.iter().enumerate() {
        find_adjacents(i, &mut adjacents);

        let adjacent_values: Vec<u8> = adjacents
            .into_iter()
            .filter(|adjacent_i| *adjacent_i <= MAX_INDEX)
            .map(|adjacent_i| heightmap.0[adjacent_i])
            .collect();

        if is_lowpoint(*value, &adjacent_values) {
            let basin_values = find_basin_indexes(i, &heightmap, &mut adjacents);
            let basin_len = basin_values.len();
            basin_lens.push(basin_len);
        }
    }

    basin_lens.sort_unstable();
    basin_lens.reverse();

    dbg!(&basin_lens[..3]);
    dbg!(basin_lens[0] * basin_lens[1] * basin_lens[2]);
}
