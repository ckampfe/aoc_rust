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

struct Graph<'a> {
    graph: HashMap<Cave<'a>, Vec<Cave<'a>>>,
}

impl<'a> Graph<'a> {
    fn paths(&self) -> Vec<Path> {
        let mut paths = vec![];

        let start_node = Cave::Small { name: "start" };
        let end_node = Cave::Small { name: "end" };

        let mut traversal_stack = vec![];

        let start_path = Path {
            nodes: vec![start_node],
            small_caves_visited: HashMap::new(),
        };

        traversal_stack.push(start_path);

        while let Some(current_path) = traversal_stack.pop() {
            if current_path.nodes.contains(&end_node) {
                paths.push(current_path);
                continue;
            }

            let current_node = current_path.nodes.last().unwrap();

            let out_nodes = self.graph.get(current_node).unwrap();

            for out_node in out_nodes {
                match out_node {
                    Cave::Small { name: "start" } => continue,
                    Cave::Small { name: "end" } => {
                        let mut clonepath = current_path.clone();
                        clonepath.nodes.push(*out_node);
                        traversal_stack.push(clonepath);
                    }
                    Cave::Small { .. } => {
                        let others_have_two = current_path
                            .small_caves_visited
                            .iter()
                            .filter(|(k, _count)| k != &out_node)
                            .any(|(_k, count)| *count >= 2);

                        let this_has_one = current_path
                            .small_caves_visited
                            .get(out_node)
                            .and_then(|count| Some(*count == 1))
                            .unwrap_or(false);

                        let this_has_two = current_path
                            .small_caves_visited
                            .get(out_node)
                            .and_then(|count| Some(*count == 2))
                            .unwrap_or(false);

                        if this_has_two || (others_have_two && this_has_one) {
                            continue;
                        } else {
                            let mut clonepath = current_path.clone();
                            let e = clonepath.small_caves_visited.entry(*out_node).or_insert(0);
                            *e += 1;
                            clonepath.nodes.push(*out_node);
                            traversal_stack.push(clonepath);
                        }
                    }
                    Cave::Large { .. } => {
                        let mut clonepath = current_path.clone();
                        clonepath.nodes.push(*out_node);
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

    let mut graph = Graph {
        graph: HashMap::new(),
    };

    for line in &file_lines {
        let edge = lex(line);

        let e0 = graph.graph.entry(edge[0]).or_insert_with(Vec::new);
        e0.push(edge[1]);

        let e1 = graph.graph.entry(edge[1]).or_insert_with(Vec::new);
        e1.push(edge[0]);
    }

    let paths = graph.paths();

    dbg!(paths.len());
}

#[derive(Clone, Debug)]
struct Path<'a> {
    nodes: Vec<Cave<'a>>,
    small_caves_visited: HashMap<Cave<'a>, u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let mut graph = Graph {
            graph: HashMap::new(),
        };

        for line in input.lines() {
            let edge = lex(line);

            let e0 = graph.graph.entry(edge[0]).or_insert_with(Vec::new);
            e0.push(edge[1]);

            let e1 = graph.graph.entry(edge[1]).or_insert_with(Vec::new);
            e1.push(edge[0]);
        }

        let paths = graph.paths();
        assert_eq!(paths.len(), 36)
    }
}
