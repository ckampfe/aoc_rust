use std::{io::BufRead, ops::Sub};

fn main() {
    let input = std::fs::File::open("inputs/7.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());
    let positions: Vec<usize> = file_lines
        .flat_map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut min_energy = energy_required(min, &positions);
    let mut min_pos = 0;
    for pos in min..max {
        let energy = energy_required(pos, &positions);
        if energy < min_energy {
            min_energy = energy;
            min_pos = pos;
        }
    }

    dbg!(min_pos);
    dbg!(min_energy);
}

fn energy_required(chosen_position: usize, positions: &[usize]) -> usize {
    let mut energy = 0;
    for position in positions {
        energy += energy_overhead(abs_difference(*position, chosen_position));
    }

    energy
}

fn abs_difference<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn energy_overhead(e: usize) -> usize {
    let mut adjusted = 0;
    for (i, _value) in (0..e).enumerate() {
        adjusted += i + 1
    }

    adjusted
}
