use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/2.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        let line = line.unwrap();
        let line = line.as_bytes();
        match line {
            [b'f', b'o', b'r', b'w', b'a', b'r', b'd', b' ', rest @ ..] => {
                let horizontal_move_str = std::str::from_utf8(rest).unwrap();
                let horizontal_move = horizontal_move_str.parse::<i32>().unwrap();
                horizontal_position += horizontal_move;
                depth += aim * horizontal_move;
            }
            [b'd', b'o', b'w', b'n', b' ', rest @ ..] => {
                let depth_str = std::str::from_utf8(rest).unwrap();
                let depth_move = depth_str.parse::<i32>().unwrap();
                aim += depth_move;
            }
            [b'u', b'p', b' ', rest @ ..] => {
                let depth_str = std::str::from_utf8(rest).unwrap();
                let depth_move = depth_str.parse::<i32>().unwrap();
                aim -= depth_move;
            }
            _ => (),
        }
    }

    println!("horizontal_position: {}", horizontal_position);
    println!("depth: {}", depth);
    println!("product: {}", horizontal_position * depth);
}
