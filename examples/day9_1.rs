use std::io::BufRead;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const MAX_WIDTH: usize = WIDTH - 1;
const MAX_HEIGHT: usize = HEIGHT - 1;

struct HeightMap(Vec<u8>);

impl HeightMap {
    fn push(&mut self, location: u8) {
        self.0.push(location);
    }
}

fn adjacents(i: usize) -> Vec<usize> {
    let (x, y) = i_to_xy(i);

    let out = match (x, y) {
        (some_x, some_y)
            if some_x > 0 && some_x < MAX_WIDTH && some_y > 0 && some_y < MAX_HEIGHT =>
        {
            vec![
                xy_to_i(some_x + 1, y),
                xy_to_i(some_x - 1, y),
                xy_to_i(some_x, y + 1),
                xy_to_i(some_x, y - 1),
            ]
        }
        (0, 0) => vec![xy_to_i(1, 0), xy_to_i(0, 1)],
        (0, MAX_HEIGHT) => vec![xy_to_i(1, MAX_HEIGHT), xy_to_i(0, MAX_HEIGHT - 1)],
        (MAX_WIDTH, 0) => vec![xy_to_i(MAX_WIDTH - 1, 0), xy_to_i(MAX_WIDTH, 1)],
        (MAX_WIDTH, MAX_HEIGHT) => vec![
            xy_to_i(MAX_WIDTH - 1, MAX_HEIGHT),
            xy_to_i(MAX_WIDTH, MAX_HEIGHT - 1),
        ],
        (MAX_WIDTH, some_y) => vec![
            xy_to_i(MAX_WIDTH, some_y + 1),
            xy_to_i(MAX_WIDTH, some_y - 1),
            xy_to_i(MAX_WIDTH - 1, some_y),
        ],
        (other_x, MAX_HEIGHT) => vec![
            xy_to_i(other_x + 1, MAX_HEIGHT),
            xy_to_i(other_x - 1, MAX_HEIGHT),
            xy_to_i(other_x, MAX_HEIGHT - 1),
        ],
        (0, some_y) => vec![
            xy_to_i(1, some_y),
            xy_to_i(0, some_y - 1),
            xy_to_i(0, some_y + 1),
        ],
        (some_x, 0) => vec![
            xy_to_i(some_x + 1, 0),
            xy_to_i(some_x - 1, 0),
            xy_to_i(some_x, 1),
        ],
        _ => panic!(),
    };

    for out_i in &out {
        let (x, y) = i_to_xy(*out_i);
        assert!(*out_i == xy_to_i(x, y));
    }

    out
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

const fn risk_level(value: u8) -> u8 {
    1 + value
}

fn is_lowpoint(value: u8, adjacent_values: &[u8]) -> bool {
    adjacent_values
        .iter()
        .all(|adjacent_value| value < *adjacent_value)
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

    let mut heightmap_lowvalue_sum: usize = 0;

    for (i, value) in heightmap.0.iter().enumerate() {
        let adjacents = adjacents(i);

        assert!(adjacents.len() == 2 || adjacents.len() == 3 || adjacents.len() == 4);

        let adjacent_values: Vec<u8> = adjacents
            .into_iter()
            .map(|adjacent_i| heightmap.0[adjacent_i])
            .collect();

        if is_lowpoint(*value, &adjacent_values) {
            heightmap_lowvalue_sum += risk_level(*value) as usize;
        }
    }

    dbg!(heightmap_lowvalue_sum);
}
