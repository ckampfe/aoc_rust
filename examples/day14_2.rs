use std::{collections::HashMap, io::BufRead};

type PairCounts = HashMap<String, usize>;
type Rules = HashMap<Matcher, Replacement>;
type Matcher = String;
type Replacement = String;

/// turn AABC into {A: 2, B: 1, C: 1, AA: 1, AB: 1, BC: 1}
fn to_pair_counts(s: &str) -> PairCounts {
    let mut out = HashMap::new();

    let chars: Vec<_> = s.chars().collect();

    for pair in chars.windows(2) {
        let mut pair_string = String::with_capacity(2);
        pair_string.push(pair[0]);
        pair_string.push(pair[1]);
        *out.entry(pair_string).or_insert(0) += 1;
    }

    for c in chars {
        let cs = c.to_string();
        *out.entry(cs).or_insert(0) += 1
    }

    out
}

fn replace(s: &PairCounts, rules: &Rules) -> PairCounts {
    let mut out = HashMap::new();

    // move letters counts over
    let letters = s.iter().filter(|(k, _v)| k.len() == 1);
    for (k, v) in letters {
        out.insert(k.to_owned(), *v);
    }

    for (pair, count) in s {
        if pair.len() > 1 {
            if let Some(replacement) = rules.get(pair) {
                let chars: Vec<_> = pair.chars().collect();

                let left_char = chars[0].to_string();
                let center_char = replacement.to_string();
                let right_char = chars[1].to_string();

                let prev_char_count = *s.get(&center_char).unwrap_or(&0);
                let e = out
                    .entry(center_char.to_string())
                    .or_insert(prev_char_count);
                *e += count;

                let all = vec![left_char, center_char, right_char];

                for new_pair in all.windows(2) {
                    let new_pair = new_pair.join("");
                    let e = out.entry(new_pair).or_insert(0);
                    *e += count;
                }
            }
        }
    }

    out
}

fn main() {
    let input = std::fs::File::open("inputs/14.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let mut file_lines = lines.map(|line| line.unwrap());

    let template = file_lines.next().unwrap();

    // skip the blank line
    file_lines.next();

    let rules = file_lines.take_while(|line| !line.is_empty());

    let rules = rules.fold(Rules::new(), |mut acc, line| {
        let split: Vec<_> = line.split(" -> ").collect();

        let matcher = split[0].to_owned();

        let replacement = split[1].to_owned();

        acc.insert(matcher, replacement);
        acc
    });

    let mut as_pair_counts = to_pair_counts(&template);

    for _step in 0..40 {
        as_pair_counts = replace(&as_pair_counts, &rules);
    }

    let chars = as_pair_counts
        .into_iter()
        .filter_map(|(k, v)| if k.len() == 1 { Some(v) } else { None });

    let mut min = usize::MAX;
    let mut max = usize::MIN;

    for char_count in chars {
        if char_count > max {
            max = char_count;
        }
        if char_count < min {
            min = char_count;
        }
    }

    dbg!(max - min);
}
