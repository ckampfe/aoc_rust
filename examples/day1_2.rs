use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/1.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let mut increases: usize = 0;

    let numbers: Vec<usize> = lines
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();

    let triples = numbers.windows(3);

    let triples_sums: Vec<usize> = triples
        .map(|triple| match triple {
            [a, b, c] => a + b + c,
            _ => panic!(),
        })
        .collect();

    let pairs = triples_sums.windows(2);

    for pair in pairs {
        match pair {
            [a, b] => {
                if b > a {
                    increases += 1;
                }
            }
            _ => panic!(),
        }
    }

    println!("{}", increases);
}
