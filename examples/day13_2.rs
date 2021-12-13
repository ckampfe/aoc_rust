use std::{fmt::Display, io::BufRead};

const XMAX: usize = 1310;
const YMAX: usize = 892;
const WIDTH: usize = XMAX + 1;
const HEIGHT: usize = YMAX + 1;

struct Paper {
    positions: [bool; WIDTH * HEIGHT],
    real_width: usize,
    real_height: usize,
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.positions.chunks(WIDTH).take(self.real_height) {
            for dot in row.iter().take(self.real_width) {
                if *dot {
                    write!(f, "# ")?;
                } else {
                    write!(f, "  ")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Paper {
    fn fold_x(&mut self, fold: usize) {
        let mut new_dots = vec![];

        for (i, is_dot) in self.positions.iter_mut().enumerate() {
            if *is_dot {
                let (x, y) = i_to_xy(i);
                if x > fold {
                    let diff = x - fold;
                    let new_x = fold - diff;
                    let new_i = xy_to_i(new_x, y);
                    new_dots.push(new_i);
                    *is_dot = false;
                }

                if x == fold {
                    *is_dot = false;
                }
            }
        }

        for i in new_dots {
            self.positions[i] = true;
        }

        self.real_width = fold;
    }

    fn fold_y(&mut self, fold: usize) {
        let mut new_dots = vec![];

        for (i, is_dot) in self.positions.iter_mut().enumerate() {
            if *is_dot {
                let (x, y) = i_to_xy(i);
                if y > fold {
                    let diff = y - fold;
                    let new_y = fold - diff;
                    let new_i = xy_to_i(x, new_y);
                    new_dots.push(new_i);
                    *is_dot = false;
                }

                if y == fold {
                    *is_dot = false;
                }
            }
        }

        for i in new_dots {
            self.positions[i] = true;
        }

        self.real_height = fold;
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
    let mut paper = Paper {
        positions: [false; WIDTH * HEIGHT],
        real_width: WIDTH,
        real_height: HEIGHT,
    };

    let input = std::fs::File::open("inputs/13.txt").unwrap();
    let dot_lines = std::io::BufReader::new(&input).lines();

    {
        let dot_lines = dot_lines
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty());

        for line in dot_lines {
            let line = line;
            let split: Vec<_> = line.split(',').collect();
            let x = split[0].parse::<usize>().unwrap();
            let y = split[1].parse::<usize>().unwrap();

            let i = xy_to_i(x, y);

            paper.positions[i] = true;
        }
    }

    {
        let input = std::fs::File::open("inputs/13.txt").unwrap();
        let fold_lines = std::io::BufReader::new(&input).lines();

        let fold_lines = fold_lines
            .map(|line| line.unwrap())
            .skip_while(|line| !line.starts_with("fold along"));

        let fold_lines: Vec<_> = fold_lines.collect();

        for fold_line in fold_lines.iter() {
            let split: Vec<_> = fold_line.split(' ').collect();
            let fold_str = split[2];
            let fold_split: Vec<_> = fold_str.split('=').collect();
            let axis = fold_split[0];
            let amount = fold_split[1];

            match axis {
                "x" => paper.fold_x(amount.parse().unwrap()),
                "y" => paper.fold_y(amount.parse().unwrap()),
                _ => panic!(),
            }
        }
    }

    dbg!(paper.real_width);
    dbg!(paper.real_height);
    println!("{}", paper.to_string())
}
