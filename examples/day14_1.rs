use std::{collections::HashMap, io::BufRead};

struct Rule {
    matcher: String,
    replacement: char,
}

fn replace(s: &str, rules: &HashMap<String, char>) -> String {
    let v: Vec<_> = s.chars().collect();
    let as_windows: Vec<_> = v
        .windows(2)
        .map(|pair| {
            let [l, r] = [pair[0], pair[1]];
            let mut s = String::with_capacity(2);
            s.push(l);
            s.push(r);
            s
        })
        .collect();

    let mut replacement_zip = vec!['X'; s.len()];

    for (i, pair) in as_windows.iter().enumerate() {
        if let Some(replacement) = rules.get(pair) {
            replacement_zip[i] = *replacement;
        }
    }

    let mut out = String::new();

    for (c1, c2) in s.chars().zip(replacement_zip) {
        out.push(c1);
        out.push(c2);
    }

    out.replace('X', "")
}

fn main() {
    let input = std::fs::File::open("inputs/14.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let mut file_lines = lines.map(|line| line.unwrap());

    let mut template = file_lines.next().unwrap();

    file_lines.next();

    let rules = file_lines.take_while(|line| !line.is_empty());
    let rules_mappings = rules.map(|line| {
        let split: Vec<_> = line.split(" -> ").collect();

        let matcher = split[0].to_owned();

        let replacement = split[1].to_owned().chars().next().unwrap();

        Rule {
            matcher,
            replacement,
        }
    });

    let mut rules = HashMap::new();

    for rule in rules_mappings {
        rules.insert(rule.matcher, rule.replacement);
    }

    for _step in 0..10 {
        template = replace(&template, &rules);
    }

    let mut counter = HashMap::new();

    for c in template.chars() {
        let e = counter.entry(c).or_insert(0);
        *e += 1;
    }

    let mut min = usize::MAX;
    let mut max = usize::MIN;

    for (k, v) in counter {
        if v > max {
            max = v;
        }
        if v < min {
            min = v;
        }
    }

    dbg!(max - min);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        // let mut file_lines = lines.map(|line| line.unwrap());
        let mut file_lines = input.split('\n');

        let mut template = file_lines.next().unwrap().to_string();

        file_lines.next();

        let rules = file_lines.take_while(|line| !line.is_empty());
        let rules_mappings = rules.map(|line| {
            let split: Vec<_> = line.split(" -> ").collect();

            let matcher = split[0].to_owned();

            let replacement = split[1].to_owned().chars().next().unwrap();

            Rule {
                matcher,
                replacement,
            }
        });

        let mut rules = HashMap::new();

        for rule in rules_mappings {
            rules.insert(rule.matcher, rule.replacement);
        }

        dbg!(&rules);

        dbg!(&template);

        for _step in 0..2 {
            template = replace(&template, &rules);
            dbg!(&template);
        }

        let mut counter = HashMap::new();

        for c in template.chars() {
            let e = counter.entry(c).or_insert(0);
            *e += 1;
        }

        let mut min = usize::MAX;
        let mut max = usize::MIN;

        for (k, v) in counter {
            if v > max {
                max = v;
            }
            if v < min {
                min = v;
            }
        }

        dbg!(max - min);

        assert!(false)
    }
}
