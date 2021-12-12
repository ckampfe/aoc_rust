use std::{collections::HashMap, io::BufRead};

type Edge<'a> = [Cave<'a>; 2];

fn lex(s: &str) -> Edge {
    let caves_strs = s.split('-');

    let mut edge = [Cave::Large { name: "Large" }; 2];

    for (i, cave) in caves_strs.enumerate() {
        edge[i] = if cave.chars().all(char::is_lowercase) {
            Cave::Small { name: cave }
        } else {
            Cave::Large { name: cave }
        };
    }

    edge
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Large { name: &'a str },
    Small { name: &'a str },
}

#[derive(Clone, Debug)]
struct Path<'a> {
    last_node: &'a Cave<'a>,
    by: u8,
    zs: u8,
    zt: u8,
    gv: u8,
    iu: u8,
}

impl<'a> Path<'a> {
    fn count(&self, s: &str) -> u8 {
        match s {
            "by" => self.by,
            "zs" => self.zs,
            "zt" => self.zt,
            "gv" => self.gv,
            "iu" => self.iu,
            other => unreachable!("{}", other),
        }
    }

    fn any_two_but(&self, s: &str) -> bool {
        match s {
            "by" => self.zs == 2 || self.zt == 2 || self.gv == 2 || self.iu == 2,
            "zs" => self.by == 2 || self.zt == 2 || self.gv == 2 || self.iu == 2,
            "zt" => self.by == 2 || self.zs == 2 || self.gv == 2 || self.iu == 2,
            "gv" => self.by == 2 || self.zs == 2 || self.zt == 2 || self.iu == 2,
            "iu" => self.by == 2 || self.zs == 2 || self.zt == 2 || self.gv == 2,
            other => unreachable!("{}", other),
        }
    }

    fn plus_one(&mut self, s: &str) {
        match s {
            "by" => self.by += 1,
            "zs" => self.zs += 1,
            "zt" => self.zt += 1,
            "gv" => self.gv += 1,
            "iu" => self.iu += 1,
            other => unreachable!("{}", other),
        }
    }
}

#[derive(Debug)]
struct CaveNetwork<'a> {
    adjacencies: HashMap<Cave<'a>, Vec<Cave<'a>>>,
}

const START_NODE: Cave = Cave::Small { name: "start" };
const END_NODE: Cave = Cave::Small { name: "end" };

impl<'a> CaveNetwork<'a> {
    fn paths(&self) -> Vec<Path> {
        let mut paths = vec![];

        let mut traversal_stack = vec![];

        let start_path = Path {
            last_node: &START_NODE,
            // small_caves_visited: HashMap::new(),
            by: 0,
            zs: 0,
            zt: 0,
            gv: 0,
            iu: 0,
        };

        traversal_stack.push(start_path);

        while let Some(current_path) = traversal_stack.pop() {
            if current_path.last_node == &END_NODE {
                paths.push(current_path);
                continue;
            }

            let out_nodes = self.adjacencies.get(current_path.last_node).unwrap();

            for out_node in out_nodes {
                match out_node {
                    Cave::Small { name: "start" } => continue,
                    Cave::Small { name: "end" } => {
                        let mut clonepath = current_path.clone();
                        clonepath.last_node = out_node;
                        traversal_stack.push(clonepath);
                    }
                    Cave::Small { name } => {
                        let others_have_two = current_path.any_two_but(name);

                        let this_visit_count = current_path.count(name);

                        if this_visit_count == 2 || (others_have_two && this_visit_count == 1) {
                            continue;
                        } else {
                            let mut clonepath = current_path.clone();
                            clonepath.plus_one(name);
                            clonepath.last_node = out_node;
                            traversal_stack.push(clonepath);
                        }
                    }
                    Cave::Large { .. } => {
                        let mut clonepath = current_path.clone();
                        clonepath.last_node = out_node;
                        traversal_stack.push(clonepath);
                    }
                }
            }
        }

        paths
    }
}

fn main() {
    let input = std::fs::File::open("inputs/12.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines: Vec<_> = lines.map(|line| line.unwrap()).collect();

    let mut cave_network = CaveNetwork {
        adjacencies: HashMap::new(),
    };

    for line in &file_lines {
        let edge = lex(line);

        let e0 = cave_network
            .adjacencies
            .entry(edge[0])
            .or_insert_with(Vec::new);
        e0.push(edge[1]);

        let e1 = cave_network
            .adjacencies
            .entry(edge[1])
            .or_insert_with(Vec::new);
        e1.push(edge[0]);
    }

    // dbg!(&cave_network);
    // panic!();

    let paths = cave_network.paths();

    dbg!(paths.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../inputs/12.txt");

        let mut graph = CaveNetwork {
            adjacencies: HashMap::new(),
        };

        for line in input.lines() {
            let edge = lex(line);

            let e0 = graph.adjacencies.entry(edge[0]).or_insert_with(Vec::new);
            e0.push(edge[1]);

            let e1 = graph.adjacencies.entry(edge[1]).or_insert_with(Vec::new);
            e1.push(edge[0]);
        }

        let paths = graph.paths();
        assert_eq!(paths.len(), 120506)
    }
}
