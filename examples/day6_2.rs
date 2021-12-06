use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/6.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());
    let fish_days: Vec<u8> = file_lines
        .flat_map(|line| {
            line.split(',')
                .map(|days_s| days_s.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut fish = [0usize; 9];

    for n in fish_days {
        fish[n as usize] += 1;
    }

    let mut all_fish = AllFish { fish };

    let mut days_elapsed = 0;

    loop {
        if days_elapsed == 256 {
            let mut count = 0usize;
            for n in all_fish.fish {
                count += n;
            }

            dbg!(count);
            return;
        }

        all_fish.tick();

        days_elapsed += 1;
    }
}

struct AllFish {
    fish: [usize; 9],
}

impl AllFish {
    fn tick(&mut self) {
        let zero = self.fish[0];

        self.fish[0] = self.fish[1];
        self.fish[1] = self.fish[2];
        self.fish[2] = self.fish[3];
        self.fish[3] = self.fish[4];
        self.fish[4] = self.fish[5];
        self.fish[5] = self.fish[6];
        self.fish[6] = self.fish[7] + zero;
        self.fish[7] = self.fish[8];
        self.fish[8] = zero;
    }
}
