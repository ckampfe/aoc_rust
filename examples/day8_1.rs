use std::{fmt::Display, io::BufRead};

// 0: 6 segments
// 1: 2 segments
// 2: 5 segments
// 3: 5 segments
// 4: 4 segments
// 5: 5 segments
// 6: 6 segments
// 7: 3 segments
// 8: 7 segments
// 9: 6 segments
// unique: 1, 4, 7, 8

#[derive(Clone, Copy, Debug)]
enum Digit<'a> {
    // Zero,
    One,
    // Two,
    // Three,
    Four,
    // Five,
    // Six,
    Seven,
    Eight,
    Unknown(&'a str),
}

impl<'a> Display for Digit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            // Digit::Zero => "0",
            Digit::One => "1",
            // Digit::Two => "2",
            // Digit::Three => "3",
            Digit::Four => "4",
            // Digit::Five => "5",
            // Digit::Six => "6",
            Digit::Seven => "7",
            Digit::Eight => "8",
            Digit::Unknown(s) => s,
        };

        write!(f, "{}", out)
    }
}

impl<'a> From<&'a str> for Digit<'a> {
    fn from(s: &'a str) -> Digit<'a> {
        match s.len() {
            2 => Digit::One,
            4 => Digit::Four,
            3 => Digit::Seven,
            7 => Digit::Eight,
            _ => Digit::Unknown(s),
        }
    }
}

#[derive(Clone, Debug)]
struct SignalPattern<'a>(&'a str);

struct Entry<'a> {
    // signal_patterns: [SignalPattern<'a>; 10],
    output: [Digit<'a>; 4],
}

fn main() {
    let input = std::fs::File::open("inputs/8.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();

    let mut entries = vec![];

    for line in file_lines.iter() {
        let split: Vec<&str> = line.split(" | ").collect();
        // let signal_patterns: [SignalPattern; 10] = split[0]
        //     .split(' ')
        //     .map(SignalPattern)
        //     .collect::<Vec<_>>()
        //     .try_into()
        //     .unwrap();

        let output: [Digit; 4] = split[1]
            .split(' ')
            .map(Digit::from)
            .collect::<Vec<Digit>>()
            .try_into()
            .unwrap();

        let entry = Entry {
            // signal_patterns,
            output,
        };

        entries.push(entry);
    }

    let mut count = 0;

    for entry in entries {
        for digit in entry.output {
            if matches!(
                digit,
                Digit::One | Digit::Four | Digit::Seven | Digit::Eight
            ) {
                count += 1;
            }
        }
    }

    dbg!(count);
}
