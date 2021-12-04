use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/1.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let mut increases: usize = 0;

    let numbers: Vec<usize> = lines
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();

    let pairs = numbers.windows(2);

    for pair in pairs {
        match pair {
            &[left, right] => {
                if right > left {
                    increases += 1;
                }
            }
            _ => panic!(),
        }
    }

    println!("{}", increases);
}
