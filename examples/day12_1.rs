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

    let mut paths = vec![];

    let start_node = Cave::Small { name: "start" };
    let end_node = Cave::Small { name: "end" };

    let mut traversal_stack = vec![];

    let start_path = Path {
        nodes: vec![start_node],
    };

    traversal_stack.push(start_path);

    while let Some(current_path) = traversal_stack.pop() {
        if current_path.nodes.contains(&end_node) {
            paths.push(current_path);
            continue;
        }

        let current_node = current_path.nodes.last().unwrap();

        let out_nodes = graph.graph.get(current_node).unwrap();

        for out_node in out_nodes {
            match out_node {
                Cave::Small { .. } => {
                    if current_path.nodes.contains(out_node) {
                        continue;
                    } else {
                        let mut clonepath = current_path.clone();
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

    for path in &paths {
        println!("{:?}", path);
    }

    dbg!(paths.len());
}

#[derive(Clone, Debug)]
struct Path<'a> {
    nodes: Vec<Cave<'a>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
