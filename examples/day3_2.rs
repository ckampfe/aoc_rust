use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/3.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();

    let lines_as_bits = lines
        .iter()
        .map(|line| {
            line.bytes()
                .map(|b| {
                    let b = &[b];
                    let b = std::str::from_utf8(b).unwrap();
                    b.parse::<u8>().unwrap()
                })
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let len = lines[0].bytes().len();
    let mut counts = vec![0; len];

    for line_bits in &lines_as_bits {
        for (i, b) in line_bits.iter().enumerate() {
            if *b == 1 {
                counts[i] += 1;
            }
        }
    }

    let mut bit_i = 0;
    let mut ox_bits = lines_as_bits.clone();

    loop {
        if ox_bits.len() == 1 {
            break;
        }

        let (zeros, ones): (Vec<Vec<u8>>, Vec<Vec<u8>>) = ox_bits
            .into_iter()
            .partition(|line_as_bits| line_as_bits[bit_i] == 0);

        ox_bits = if zeros.len() == ones.len() {
            ones
        } else {
            if zeros.len() < ones.len() {
                ones
            } else {
                zeros
            }
        };

        bit_i += 1;
    }

    let mut bit_i = 0;
    let mut co2_bits = lines_as_bits.clone();

    loop {
        if co2_bits.len() == 1 {
            break;
        }

        let (zeros, ones): (Vec<Vec<u8>>, Vec<Vec<u8>>) = co2_bits
            .into_iter()
            .partition(|line_as_bits| line_as_bits[bit_i] == 0);

        co2_bits = if zeros.len() == ones.len() {
            zeros
        } else {
            if zeros.len() < ones.len() {
                zeros
            } else {
                ones
            }
        };

        bit_i += 1;
    }

    let mut ox = 0;
    let mut co2 = 0;

    for (i, b) in ox_bits[0].iter().rev().enumerate() {
        if *b == 1 {
            ox += 2usize.pow(i as u32) as usize
        }
    }

    for (i, b) in co2_bits[0].iter().rev().enumerate() {
        if *b == 1 {
            co2 += 2usize.pow(i as u32) as usize
        }
    }

    println!("ox {}", ox);
    println!("co2 {}", co2);
    println!("product {}", ox * co2);
}
