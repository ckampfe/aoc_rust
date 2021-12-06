use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("inputs/6.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());
    let mut all_fish: Vec<Fish> = file_lines
        .flat_map(|line| {
            line.split(",")
                .map(|days_s| {
                    let days = days_s.parse::<u8>().unwrap();
                    Fish::new(days)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    println!("initial fish len {}", all_fish.len());

    let mut days_elapsed = 0;
    let mut todays_fish = vec![];

    loop {
        if days_elapsed == 80 {
            println!("{}", all_fish.len());
            return;
        }

        for fish in &mut all_fish {
            if let Some(new_fish) = fish.tick() {
                todays_fish.push(new_fish)
            }
        }

        all_fish.extend_from_slice(&todays_fish);
        todays_fish.clear();

        days_elapsed += 1;
    }
}

#[derive(Clone, Debug)]
struct Fish {
    is_new: bool,
    days_remaining: u8,
}

impl Fish {
    fn new(days_remaining: u8) -> Self {
        Self {
            is_new: false,
            days_remaining,
        }
    }

    fn tick(&mut self) -> Option<Fish> {
        if self.days_remaining == 0 {
            self.days_remaining = 6;
            self.is_new = false;
            return Some(Fish::default());
        } else {
            self.days_remaining -= 1;
            None
        }
    }
}

impl Default for Fish {
    fn default() -> Self {
        Self {
            is_new: true,
            days_remaining: 8,
        }
    }
}
