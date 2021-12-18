use std::io::BufRead;

#[derive(Clone, Debug, PartialEq)]
struct FlatNumbers {
    values: Vec<usize>,
    depths: Vec<usize>,
}

impl FlatNumbers {
    fn add(&mut self, other: &Self) {
        self.values.extend(other.values.iter());
        self.depths.extend(other.depths.iter());
        for depth in self.depths.iter_mut() {
            *depth += 1;
        }

        self.reduce()
    }

    fn magnitude(&mut self) -> usize {
        while self.magnitude_once() {}
        self.values[0]
    }

    fn reduce(&mut self) {
        loop {
            // check explodes
            if self.explode_once() {
                continue;
            };

            // check split
            if self.split_once() {
                continue;
            }

            break;
        }
    }

    fn magnitude_once(&mut self) -> bool {
        if let Some((i, _window)) = self.depths.windows(2).enumerate().find(|(_i, window)| {
            assert!(window.len() == 2);
            window[0] == window[1]
        }) {
            let left = self.values[i] * 3;
            let right = self.values[i + 1] * 2;
            self.depths.remove(i + 1);
            self.depths[i] -= 1;
            self.values.remove(i + 1);
            self.values[i] = left + right;
            true
        } else {
            false
        }
    }

    fn split_once(&mut self) -> bool {
        if let Some((i, v)) = self.values.iter().enumerate().find(|(_i, v)| **v >= 10) {
            let left = ((*v as f32) / 2.0).floor() as usize;
            let right = ((*v as f32) / 2.0).ceil() as usize;
            let depth = self.depths[i] + 1;
            self.values[i] = right;
            self.values.insert(i, left);
            self.depths[i] = depth;
            self.depths.insert(i, depth);
            true
        } else {
            false
        }
    }

    fn explode_once(&mut self) -> bool {
        let mut explode_i = None;
        for (i, depth_chunk) in self.depths.windows(2).enumerate() {
            assert!(depth_chunk.len() == 2);

            if depth_chunk[0] >= 5 && (depth_chunk[0] == depth_chunk[1]) {
                explode_i = Some(i);
                break;
            }
        }

        if let Some(i) = explode_i {
            let lval = self.values[i];
            let rval = self.values[i + 1];

            if i == 0 {
                self.values[i + 2] += rval;

                self.values.remove(i + 1);
                self.depths.remove(i + 1);
                self.values[i] = 0;
                self.depths[i] -= 1;
                return true;
            }
            if i + 1 == self.values.len() - 1 {
                self.values[i - 1] += lval;

                self.values.remove(i + 1);
                self.depths.remove(i + 1);
                self.values[i] = 0;
                self.depths[i] -= 1;
                return true;
            }

            self.values[i - 1] += lval;
            self.values[i + 2] += rval;

            self.values.remove(i + 1);
            self.depths.remove(i + 1);
            self.values[i] = 0;
            self.depths[i] -= 1;

            true
        } else {
            false
        }
    }
}

fn lex_flat(line: &str) -> FlatNumbers {
    let mut flat_numbers = FlatNumbers {
        values: vec![],
        depths: vec![],
    };

    let mut depth = 0;

    let mut this_number_buf = vec![];

    for b in line.bytes() {
        match b {
            b'[' => depth += 1,
            b']' => {
                if !this_number_buf.is_empty() {
                    let as_str = std::str::from_utf8(&this_number_buf).unwrap();
                    let value = as_str.parse::<usize>().unwrap();
                    this_number_buf.clear();

                    flat_numbers.values.push(value);
                    flat_numbers.depths.push(depth);
                }

                depth -= 1
            }
            b',' => {
                if !this_number_buf.is_empty() {
                    let as_str = std::str::from_utf8(&this_number_buf).unwrap();
                    let value = as_str.parse::<usize>().unwrap();
                    this_number_buf.clear();

                    flat_numbers.values.push(value);
                    flat_numbers.depths.push(depth);
                }
            }
            b'0'..=b'9' => this_number_buf.push(b),
            _ => (),
        }
    }

    flat_numbers
}

fn main() {
    let input = std::fs::File::open("inputs/18.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();
    let lines: Vec<_> = lines
        .map(|line| line.unwrap())
        .map(|line| lex_flat(&line))
        .collect();

    let mut max_magnitude = usize::MIN;

    for (j, n1) in lines.iter().enumerate() {
        for (i, n2) in lines.iter().enumerate() {
            if i != j {
                let mut n1 = n1.clone();
                let n1fresh = n1.clone();
                let mut n2 = n2.clone();

                n1.add(&n2);
                let n1_mag = n1.magnitude();
                if n1_mag > max_magnitude {
                    max_magnitude = n1_mag;
                }

                n2.add(&n1fresh);
                let n2_mag = n2.magnitude();
                if n2_mag > max_magnitude {
                    max_magnitude = n2_mag;
                }
            }
        }
    }

    dbg!(max_magnitude);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explode() {
        let input = "[[[[[9,8],1],2],3],4]";
        let expected = "[[[[0,9],2],3],4]";
        let mut input_number = lex_flat(input);
        let expected_number = lex_flat(expected);

        input_number.explode_once();

        assert_eq!(input_number, expected_number);

        let input = "[7,[6,[5,[4,[3,2]]]]]";
        let expected = "[7,[6,[5,[7,0]]]]";
        let mut input_number = lex_flat(input);
        let expected_number = lex_flat(expected);

        input_number.explode_once();

        assert_eq!(input_number, expected_number);

        let input = "[[6,[5,[4,[3,2]]]],1]";
        let expected = "[[6,[5,[7,0]]],3]";
        let mut input_number = lex_flat(input);
        let expected_number = lex_flat(expected);

        input_number.explode_once();

        assert_eq!(input_number, expected_number);

        let input = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let expected = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        let mut input_number = lex_flat(input);
        let expected_number = lex_flat(expected);

        input_number.explode_once();

        assert_eq!(input_number, expected_number);
    }

    #[test]
    fn split() {
        let input = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let expected = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]";
        let mut input_number = lex_flat(input);
        let expected_number = lex_flat(expected);

        input_number.split_once();

        assert_eq!(input_number, expected_number);

        let expected = "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]";
        let expected_number = lex_flat(expected);

        input_number.split_once();

        assert_eq!(input_number, expected_number);
    }

    #[test]
    fn flatten() {
        let input = "[[[[[9,8],1],2],3],4]";
        let input_number = lex_flat(input);

        assert_eq!(
            input_number,
            FlatNumbers {
                values: vec![9, 8, 1, 2, 3, 4],
                depths: vec![5, 5, 4, 3, 2, 1]
            }
        );

        let input = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        let input_number = lex_flat(input);
        assert_eq!(
            input_number,
            FlatNumbers {
                values: vec![3, 2, 8, 0, 9, 5, 7, 0],
                depths: vec![2, 3, 4, 4, 2, 3, 4, 4]
            }
        );

        let input = "[[[[0,7],4],[15,[0,13]]],[1,1]]";
        let input_number = lex_flat(input);
        assert_eq!(
            input_number,
            FlatNumbers {
                values: vec![0, 7, 4, 15, 0, 13, 1, 1],
                depths: vec![4, 4, 3, 3, 4, 4, 2, 2]
            }
        )
    }

    #[test]
    fn add() {
        let input = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let mut input_number = lex_flat(input);
        let add = "[1,1]";
        let add_number = lex_flat(add);
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let expected_number = lex_flat(expected);

        input_number.add(&add_number);

        assert_eq!(input_number, expected_number);
    }

    #[test]
    fn magnitude() {
        let input = "[9,1]";
        let mut input_number = lex_flat(input);
        let magnitude = input_number.magnitude();
        assert_eq!(magnitude, 29);

        let input = "[1,9]";
        let mut input_number = lex_flat(input);
        let magnitude = input_number.magnitude();
        assert_eq!(magnitude, 21);

        let input = "[[9,1],[1,9]]";
        let mut input_number = lex_flat(input);
        let magnitude = input_number.magnitude();
        assert_eq!(magnitude, 129);

        let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        let mut input_number = lex_flat(input);
        let magnitude = input_number.magnitude();
        assert_eq!(magnitude, 3488);
    }
}
