use std::{collections::HashMap, io::BufRead};

fn main() {
    let input = std::fs::File::open("inputs/5.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());

    let mut lines = vec![];

    for line in file_lines {
        let point_strings: Vec<_> = line.split(" -> ").collect();
        let mut points = vec![];
        for point_string in point_strings {
            let xy_string: Vec<_> = point_string.split(",").collect();
            let point = Point {
                x: xy_string[0].parse().unwrap(),
                y: xy_string[1].parse().unwrap(),
            };
            points.push(point);
        }

        lines.push(Line {
            a: points[0],
            b: points[1],
        })
    }

    let horizontal_and_vertical = lines.iter().filter(|line| line.is_horizontal_or_vertical());

    let mut counts = HashMap::new();

    for line in horizontal_and_vertical {
        let points = line.all_points_on();
        for point in points {
            let entry = counts.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    let with_at_least_2_intersections = counts
        .into_iter()
        .filter(|(_k, v)| *v >= 2i32)
        .map(|_| 1usize)
        .sum::<usize>();

    dbg!(with_at_least_2_intersections);
}

#[derive(Clone, Copy, Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }

    fn all_points_on(&self) -> Vec<Point> {
        let mut points = vec![];
        if self.a.x == self.b.x {
            let lower = self.a.y.min(self.b.y);
            let upper = self.a.y.max(self.b.y);
            for n in lower..=upper {
                points.push(Point { x: self.a.x, y: n })
            }
        } else {
            let lower = self.a.x.min(self.b.x);
            let upper = self.a.x.max(self.b.x);
            for n in lower..=upper {
                points.push(Point { x: n, y: self.a.y })
            }
        }

        points
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
