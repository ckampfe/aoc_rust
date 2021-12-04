use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/3.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let mut linespeek = lines.peekable();
    let len = linespeek.peek().unwrap().as_ref().unwrap().bytes().len();
    let mut counts = vec![0; len];
    let mut total_length = 0;

    for line in linespeek {
        let line = line.unwrap();
        for (i, b) in line.bytes().enumerate() {
            let b = &[b];
            let b = std::str::from_utf8(b).unwrap();
            let b = b.parse::<u8>().unwrap();
            if b == 1 {
                counts[i] += 1;
            }
        }

        total_length += 1;
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;

    dbg!(&counts);
    dbg!(total_length);

    for b in counts.iter() {
        if *b > total_length / 2 {
            println!("1");
        } else {
            println!("0");
        }
    }

    for (i, count) in counts.iter().rev().enumerate() {
        if *count > (total_length / 2) {
            println!("{}", 1 << i);
            gamma += 2u32.pow(i as u32) as usize
        } else {
            epsilon += 2u32.pow(i as u32) as usize
        }
    }

    dbg!(gamma);
    dbg!(epsilon);
    dbg!(gamma * epsilon);
}
