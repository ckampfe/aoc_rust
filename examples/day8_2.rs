use std::{collections::BTreeSet, fmt::Display, io::BufRead, ops::BitXor};

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
//
// all lengths: 2, 3, 4, 5, 6, 7
// uniqueN: 1, 4, 7, 8
// nonuniqueN: 0, 2, 3, 5, 6, 9
// len2: (1)
// len3: (7)
// len4: (4)
// len5: (2|3|5)
// len6: (0|6|9)
// len7: (8)

#[derive(Clone, Debug)]
struct Entry {
    signal_patterns: [SignalPattern; 10],
    output: [SignalPattern; 4],
}

#[derive(Clone, Debug)]
enum SignalPattern {
    Known {
        digit: Digit,
        segments: BTreeSet<Segment>,
    },
    Unknown {
        segments: BTreeSet<Segment>,
    },
}

impl SignalPattern {
    fn segments(&self) -> &BTreeSet<Segment> {
        match self {
            SignalPattern::Known { segments, .. } => segments,
            SignalPattern::Unknown { segments, .. } => segments,
        }
    }

    fn deduce(&mut self, patterns: &[SignalPattern]) {
        match self {
            SignalPattern::Known { .. } => (),
            SignalPattern::Unknown { segments, .. } => {
                let one_four_seven_eight = patterns.iter().filter(|pattern| {
                    matches!(
                        pattern,
                        SignalPattern::Known {
                            digit: Digit::One,
                            ..
                        } | SignalPattern::Known {
                            digit: Digit::Four,
                            ..
                        } | SignalPattern::Known {
                            digit: Digit::Seven,
                            ..
                        } | SignalPattern::Known {
                            digit: Digit::Eight,
                            ..
                        }
                    )
                });

                let mut xors: [usize; 4] = one_four_seven_eight
                    .map(|known_pattern| known_pattern.segments().bitxor(segments))
                    .map(|bxor| bxor.len())
                    .collect::<Vec<usize>>()
                    .try_into()
                    .unwrap();

                xors.sort_unstable();

                let out = match xors {
                    [2, 2, 3, 3] => SignalPattern::Known {
                        digit: Digit::Three,
                        segments: segments.clone(),
                    },
                    [2, 4, 5, 5] => SignalPattern::Known {
                        digit: Digit::Two,
                        segments: segments.clone(),
                    },
                    [2, 3, 4, 5] => SignalPattern::Known {
                        digit: Digit::Five,
                        segments: segments.clone(),
                    },
                    [1, 3, 4, 4] => SignalPattern::Known {
                        digit: Digit::Zero,
                        segments: segments.clone(),
                    },
                    [1, 4, 5, 6] => SignalPattern::Known {
                        digit: Digit::Six,
                        segments: segments.clone(),
                    },
                    [1, 2, 3, 4] => SignalPattern::Known {
                        digit: Digit::Nine,
                        segments: segments.clone(),
                    },
                    _ => panic!("{:?} did not match 2, 3, or 5", xors),
                };

                *self = out;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

fn main() {
    let input = std::fs::File::open("inputs/8.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();

    let mut entries = vec![];

    for line in file_lines.iter() {
        let splits: Vec<&str> = line.split(" | ").collect();

        let signal_patterns: [SignalPattern; 10] = splits[0]
            .split(' ')
            .map(SignalPattern::from)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let output: [SignalPattern; 4] = splits[1]
            .split(' ')
            .map(SignalPattern::from)
            .collect::<Vec<SignalPattern>>()
            .try_into()
            .unwrap();

        let entry = Entry {
            signal_patterns,
            output,
        };

        entries.push(entry);
    }

    let mut output_total = 0;

    for entry in entries.iter_mut() {
        // println!("before: {:?}", entry.to_string());

        let ec = entry.signal_patterns.clone();

        for signal_pattern in entry.signal_patterns.iter_mut() {
            signal_pattern.deduce(&ec);
        }

        for output_pattern in entry.output.iter_mut() {
            *output_pattern = entry
                .signal_patterns
                .iter()
                .cloned()
                .find(|sp| sp.segments() == output_pattern.segments())
                .unwrap();
        }

        let mut digits_s = String::new();

        for output_digit in entry.output.iter() {
            match output_digit {
                SignalPattern::Known { digit, .. } => digits_s.push_str(&digit.to_string()),
                _ => unreachable!(),
            }
        }

        let n = digits_s.parse::<usize>().unwrap();

        output_total += n;

        // println!("after: {:?}", entry.to_string());
    }

    dbg!(output_total);
}

//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
// Everything after this is impls for parsing/displaying/debugging
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] | [{}]",
            self.signal_patterns
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            self.output
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

impl From<&str> for SignalPattern {
    fn from(s: &str) -> Self {
        let segments = s.chars().map(Segment::from).collect();

        match Digit::try_from(s) {
            Ok(digit) => SignalPattern::Known { digit, segments },
            Err(_) => SignalPattern::Unknown { segments },
        }
    }
}

impl Display for SignalPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalPattern::Known { digit, segments } => write!(
                f,
                "{}({})",
                digit.to_string(),
                segments
                    .iter()
                    .map(|segment| segment.to_string())
                    .collect::<Vec<_>>()
                    .join(""),
            ),
            SignalPattern::Unknown { segments } => write!(
                f,
                "({})",
                segments
                    .iter()
                    .map(|segment| segment.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ),
        }
    }
}

impl TryFrom<&str> for Digit {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.len() {
            2 => Ok(Digit::One),
            4 => Ok(Digit::Four),
            3 => Ok(Digit::Seven),
            7 => Ok(Digit::Eight),
            _ => Err(()),
        }
    }
}

impl<'a> Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Digit::Zero => "0",
            Digit::One => "1",
            Digit::Two => "2",
            Digit::Three => "3",
            Digit::Four => "4",
            Digit::Five => "5",
            Digit::Six => "6",
            Digit::Seven => "7",
            Digit::Eight => "8",
            Digit::Nine => "9",
        };

        write!(f, "{}", out)
    }
}

impl From<Digit> for usize {
    fn from(digit: Digit) -> Self {
        match digit {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}

impl From<char> for Segment {
    fn from(c: char) -> Self {
        match c {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => panic!(),
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Segment::A => "a",
            Segment::B => "b",
            Segment::C => "c",
            Segment::D => "d",
            Segment::E => "e",
            Segment::F => "f",
            Segment::G => "g",
        };

        write!(f, "{}", out)
    }
}
