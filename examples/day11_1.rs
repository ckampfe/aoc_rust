use std::collections::BTreeSet;
use std::fmt::Display;
use std::io::BufRead;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const MAX_WIDTH: usize = WIDTH - 1;
const MAX_HEIGHT: usize = HEIGHT - 1;

struct EnergyMap {
    map: [u8; WIDTH * HEIGHT],
}

impl EnergyMap {
    fn tick(&mut self) -> usize {
        // First, the energy level of each octopus increases by 1.
        for energy_level in self.map.iter_mut() {
            *energy_level += 1;
        }

        // Then, any octopus with an energy level greater than 9 flashes.
        // This increases the energy level of all adjacent octopuses by 1,
        // including octopuses that are diagonally adjacent.
        // If this causes an octopus to have an energy level greater than 9,
        // it also flashes.
        // This process continues as long as new octopuses keep having their
        // energy level increased beyond 9.
        // (An octopus can only flash at most once per step.)
        let mut most_recent_flashed: BTreeSet<_> = self
            .map
            .iter()
            .enumerate()
            .filter(|(_i, energy)| **energy > 9)
            .map(|(i, _energy)| i)
            .collect();

        let mut all_flashed: BTreeSet<usize> = most_recent_flashed.clone();

        loop {
            let all_adjacents: Vec<usize> = most_recent_flashed
                .iter()
                .flat_map(|i| find_adjacents(*i))
                .filter(|i| !all_flashed.contains(i))
                .collect();

            for adjacent in &all_adjacents {
                self.map[*adjacent] += 1;
            }

            let unique_adjacents: BTreeSet<_> = all_adjacents.into_iter().collect();

            let adjacent_flashes: Vec<_> = unique_adjacents
                .iter()
                .filter(|i| self.map[**i] > 9)
                .collect();

            all_flashed.extend(adjacent_flashes.iter().copied().copied());

            if adjacent_flashes.is_empty() {
                break;
            } else {
                most_recent_flashed = adjacent_flashes.iter().copied().copied().collect();
            }
        }

        // Finally, any octopus that flashed during this step has its energy level set to 0,
        // as it used all of its energy to flash.
        for i in &all_flashed {
            self.map[*i] = 0;
        }

        all_flashed.len()
    }
}

impl Display for EnergyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.map.chunks(10) {
            writeln!(f, "{:?}", chunk)?
        }

        Ok(())
    }
}

fn find_adjacents(i: usize) -> Vec<usize> {
    let (x, y) = i_to_xy(i);

    match (x, y) {
        // regular case
        (some_x, some_y)
            if some_x > 0 && some_x < MAX_WIDTH && some_y > 0 && some_y < MAX_HEIGHT =>
        {
            vec![
                // horizontal and vertical
                xy_to_i(some_x + 1, y),
                xy_to_i(some_x - 1, y),
                xy_to_i(some_x, y + 1),
                xy_to_i(some_x, y - 1),
                // diagonals
                xy_to_i(some_x + 1, y + 1),
                xy_to_i(some_x + 1, y - 1),
                xy_to_i(some_x - 1, y + 1),
                xy_to_i(some_x - 1, y - 1),
            ]
        }
        // upper left
        (0, 0) => {
            vec![xy_to_i(1, 0), xy_to_i(0, 1), xy_to_i(1, 1)]
        }
        // lower left
        (0, MAX_HEIGHT) => {
            vec![
                xy_to_i(1, MAX_HEIGHT),
                xy_to_i(0, MAX_HEIGHT - 1),
                xy_to_i(1, MAX_HEIGHT - 1),
            ]
        }
        // upper right
        (MAX_WIDTH, 0) => {
            vec![
                xy_to_i(MAX_WIDTH - 1, 0),
                xy_to_i(MAX_WIDTH, 1),
                xy_to_i(MAX_WIDTH - 1, 1),
            ]
        }
        // lower right
        (MAX_WIDTH, MAX_HEIGHT) => {
            vec![
                xy_to_i(MAX_WIDTH - 1, MAX_HEIGHT),
                xy_to_i(MAX_WIDTH, MAX_HEIGHT - 1),
                xy_to_i(MAX_WIDTH - 1, MAX_HEIGHT - 1),
            ]
        }
        // right column
        (MAX_WIDTH, some_y) => {
            vec![
                xy_to_i(MAX_WIDTH, some_y + 1),
                xy_to_i(MAX_WIDTH, some_y - 1),
                xy_to_i(MAX_WIDTH - 1, some_y),
                xy_to_i(MAX_WIDTH - 1, some_y + 1),
                xy_to_i(MAX_WIDTH - 1, some_y - 1),
            ]
        }
        // bottom row
        (some_x, MAX_HEIGHT) => {
            vec![
                xy_to_i(some_x + 1, MAX_HEIGHT),
                xy_to_i(some_x - 1, MAX_HEIGHT),
                xy_to_i(some_x, MAX_HEIGHT - 1),
                xy_to_i(some_x + 1, MAX_HEIGHT - 1),
                xy_to_i(some_x - 1, MAX_HEIGHT - 1),
            ]
        }
        // left column
        (0, some_y) => {
            vec![
                xy_to_i(1, some_y),
                xy_to_i(0, some_y - 1),
                xy_to_i(0, some_y + 1),
                xy_to_i(1, some_y - 1),
                xy_to_i(1, some_y + 1),
            ]
        }
        // top row
        (some_x, 0) => {
            vec![
                xy_to_i(some_x + 1, 0),
                xy_to_i(some_x - 1, 0),
                xy_to_i(some_x, 1),
                xy_to_i(some_x + 1, 1),
                xy_to_i(some_x - 1, 1),
            ]
        }
        _ => unreachable!("unhandled index case"),
    }
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

fn main() {
    let input = std::fs::File::open("inputs/11.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());

    let mut energy_impl = [u8::default(); WIDTH * HEIGHT];

    for (y, line) in file_lines.enumerate() {
        for (x, value_at_location) in line.split("").filter(|s| !s.is_empty()).enumerate() {
            let value_at_location = value_at_location.parse::<u8>().unwrap();
            energy_impl[xy_to_i(x, y)] = value_at_location;
        }
    }

    let mut energy_map: EnergyMap = EnergyMap { map: energy_impl };

    let mut total_flashes = 0;

    for _tick in 0..100 {
        total_flashes += energy_map.tick();
    }

    dbg!(total_flashes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = [
            5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
            3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
            4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
            5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
        ];

        let after_1_tick = [
            6, 5, 9, 4, 2, 5, 4, 3, 3, 4, 3, 8, 5, 6, 9, 6, 5, 8, 2, 2, 6, 3, 7, 5, 6, 6, 7, 2, 8,
            4, 7, 2, 5, 2, 4, 4, 7, 2, 5, 7, 7, 4, 6, 8, 4, 9, 6, 5, 8, 9, 5, 2, 7, 8, 6, 3, 5, 7,
            5, 6, 3, 2, 8, 7, 9, 5, 2, 8, 3, 2, 7, 9, 9, 3, 9, 9, 2, 2, 4, 5, 5, 9, 5, 7, 9, 5, 9,
            6, 6, 5, 6, 3, 9, 4, 8, 6, 2, 6, 3, 7,
        ];

        let after_2_tick = [
            8, 8, 0, 7, 4, 7, 6, 5, 5, 5, 5, 0, 8, 9, 0, 8, 7, 0, 5, 4, 8, 5, 9, 7, 8, 8, 9, 6, 0,
            8, 8, 4, 8, 5, 7, 6, 9, 6, 0, 0, 8, 7, 0, 0, 9, 0, 8, 8, 0, 0, 6, 6, 0, 0, 0, 8, 8, 9,
            8, 9, 6, 8, 0, 0, 0, 0, 5, 9, 4, 3, 0, 0, 0, 0, 0, 0, 7, 4, 5, 6, 9, 0, 0, 0, 0, 0, 0,
            8, 7, 6, 8, 7, 0, 0, 0, 0, 6, 8, 4, 8,
        ];

        let mut energy_map = EnergyMap { map: input };

        energy_map.tick();

        assert_eq!(energy_map.map, after_1_tick);

        energy_map.tick();

        assert_eq!(energy_map.map, after_2_tick);
    }
}
