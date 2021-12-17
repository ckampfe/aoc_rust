use std::{collections::HashSet, io::BufRead};

#[derive(Clone, Copy, Debug)]
struct Probe {
    position: Position,
    velocity: Velocity,
}

impl Probe {
    fn lands_within_target_area(&mut self, target_area: &TargetArea) -> Lands {
        loop {
            self.step();

            if target_area.is_within(self) {
                return Lands::Within;
            }

            if self.position.x > target_area.xmax {
                return Lands::Right;
            }

            if self.position.x < target_area.xmin && self.position.y < target_area.ymin {
                return Lands::Left;
            }

            if self.position.y < target_area.ymin {
                return Lands::Below;
            }
        }
    }

    fn step(&mut self) {
        // The probe's x position increases by its x velocity.
        self.position.x += self.velocity.x;
        // The probe's y position increases by its y velocity.
        self.position.y += self.velocity.y;

        // Due to drag, the probe's x velocity changes by 1 toward the value 0;
        // that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
        match self.velocity.x.cmp(&0) {
            std::cmp::Ordering::Greater => {
                self.velocity.x -= 1;
            }
            std::cmp::Ordering::Less => {
                self.velocity.x += 1;
            }
            std::cmp::Ordering::Equal => (),
        }

        // Due to gravity, the probe's y velocity decreases by 1.
        self.velocity.y -= 1
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Velocity {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, Debug, Default)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug)]
struct TargetArea {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

impl TargetArea {
    fn is_within(&self, probe: &Probe) -> bool {
        probe.position.x >= self.xmin
            && probe.position.x <= self.xmax
            && probe.position.y >= self.ymin
            && probe.position.y <= self.ymax
    }
}

enum Lands {
    Left,
    Right,
    Below,
    Within,
}

fn find_good_shots(
    target_area: &TargetArea,
    xmin: isize,
    ymin: isize,
    xmax: isize,
    ymax: isize,
) -> HashSet<Velocity> {
    let mut good_shots = HashSet::new();

    for x in xmin..=xmax {
        for y in ymin..=ymax {
            let v = Velocity {
                x: x as isize,
                y: y as isize,
            };
            let mut probe = Probe {
                position: Position::default(),
                velocity: v,
            };

            if let Lands::Within = probe.lands_within_target_area(target_area) {
                good_shots.insert(v);
            }
        }
    }

    good_shots
}

fn main() {
    let input = std::fs::File::open("inputs/17.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    // target area: x=153..199, y=-114..-75
    let file_lines: Vec<String> = lines.map(|line| line.unwrap()).take(1).collect();
    let target_area_s = file_lines[0].clone();
    let target_area_s = target_area_s.replace(',', "");
    let target_area_s = target_area_s.replace("target area: ", "");
    let split: Vec<_> = target_area_s.split(' ').collect();

    let xs = split[0];
    let xs = xs.replace("x=", "");
    let xs: Vec<&str> = xs.split("..").collect();
    let xmin = xs[0].parse().unwrap();
    let xmax = xs[1].parse().unwrap();

    let ys = split[1];
    let ys = ys.replace("y=", "");
    let ys: Vec<&str> = ys.split("..").collect();
    let ymin = ys[0].parse().unwrap();
    let ymax = ys[1].parse().unwrap();

    let target_area = TargetArea {
        xmin,
        xmax,
        ymin,
        ymax,
    };

    let good_shots = find_good_shots(&target_area, 0, target_area.ymin, target_area.xmax, 6441);

    dbg!(good_shots.len());
}
