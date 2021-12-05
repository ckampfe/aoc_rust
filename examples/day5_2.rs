use std::{collections::HashMap, io::BufRead};

fn main() {
    let input = std::fs::File::open("inputs/5.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());

    let mut lines = Vec::with_capacity(500);
    let mut points: [Point; 2] = [Default::default(); 2];

    for line in file_lines {
        let point_strings: Vec<_> = line.split(" -> ").collect();

        for (i, point_string) in point_strings.iter().enumerate() {
            let xy_string: Vec<_> = point_string.split(',').collect();
            let point = Point {
                x: xy_string[0].parse().unwrap(),
                y: xy_string[1].parse().unwrap(),
            };

            points[i] = point;
        }

        lines.push(Line {
            p1: points[0],
            p2: points[1],
        })
    }

    let mut counts = HashMap::new();

    for line in lines {
        let points = line.all_points_on();
        for point in points {
            let entry = counts.entry(point).or_insert(0usize);
            *entry += 1;
        }
    }

    let with_at_least_2_intersections = counts
        .into_iter()
        .filter(|(_k, v)| *v >= 2usize)
        .fold(0, |acc, _| acc + 1);

    dbg!(with_at_least_2_intersections);
}

#[derive(Clone, Copy, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn all_points_on(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        if self.p1.x == self.p2.x {
            let lower = self.p1.y.min(self.p2.y);
            let upper = self.p1.y.max(self.p2.y);

            Box::new((lower..=upper).map(|n| Point { x: self.p1.x, y: n }))
        } else if self.p1.y == self.p2.y {
            let lower = self.p1.x.min(self.p2.x);
            let upper = self.p1.x.max(self.p2.x);

            Box::new((lower..=upper).map(|n| Point { x: n, y: self.p1.y }))
        } else {
            let xrange: Box<dyn Iterator<Item = usize>> = if self.p1.x <= self.p2.x {
                Box::new(self.p1.x..=self.p2.x)
            } else {
                Box::new((self.p2.x..=self.p1.x).rev())
            };

            let yrange: Box<dyn Iterator<Item = usize>> = if self.p1.y <= self.p2.y {
                Box::new(self.p1.y..=self.p2.y)
            } else {
                Box::new((self.p2.y..=self.p1.y).rev())
            };

            Box::new(xrange.into_iter().zip(yrange).map(|(x, y)| Point { x, y }))
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagonals() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 2, y: 2 };
        let line = Line { p1, p2 };
        let mut expected = vec![p1, p2, Point { x: 1, y: 1 }];
        expected.sort_unstable();
        let mut result = line.all_points_on().collect::<Vec<_>>();
        result.sort_unstable();
        assert_eq!(result, expected);

        let p1 = Point { x: 2, y: 2 };
        let p2 = Point { x: 0, y: 0 };
        let line = Line { p1, p2 };
        let mut expected = vec![p1, p2, Point { x: 1, y: 1 }];
        expected.sort_unstable();
        let mut result = line.all_points_on().collect::<Vec<_>>();
        result.sort_unstable();
        assert_eq!(result, expected);

        let p1 = Point { x: 0, y: 2 };
        let p2 = Point { x: 2, y: 0 };
        let line = Line { p1, p2 };
        let mut expected = vec![p1, p2, Point { x: 1, y: 1 }];
        expected.sort_unstable();
        let mut result = line.all_points_on().collect::<Vec<_>>();
        result.sort_unstable();
        assert_eq!(result, expected);
    }
}
