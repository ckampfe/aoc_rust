use std::io::BufRead;

const XMAX: usize = 1310;
const YMAX: usize = 892;
const WIDTH: usize = XMAX + 1;
const HEIGHT: usize = YMAX + 1;

struct Paper {
    positions: [bool; WIDTH * HEIGHT],
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
            .skip_while(|line| !line.starts_with("fold along"))
            .take(1);

        for fold_line in fold_lines {
            let split: Vec<_> = fold_line.split(' ').collect();
            let fold_str = split[2];
            let fold_split: Vec<_> = fold_str.split('=').collect();
            let axis = fold_split[0];
            let amount = fold_split[1];

            match axis {
                "x" => paper.fold_x(amount.parse().unwrap()),
                "y" => {
                    todo!("don't need y for just the first fold")
                }
                _ => {
                    panic!("why")
                }
            }
        }

        let mut visible_count: usize = 0;

        for is_visible in paper.positions.iter() {
            if *is_visible {
                visible_count += 1;
            }
        }

        dbg!(visible_count);
    }
}
