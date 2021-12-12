use std::{collections::HashMap, io::BufRead, str::FromStr};

type Edge = [Cave; 2];

fn lex(s: &str) -> Edge {
    let caves_strs = s.split('-');

    let mut edge = [Cave::Start; 2];

    for (i, cave) in caves_strs.enumerate() {
        edge[i] = Cave::from_str(cave).unwrap();
    }

    edge
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Cave {
    // Special
    Start,
    End,
    // Small
    By,
    Zs,
    Zt,
    Gv,
    Iu,
    // Big
    Wo,
    Qj,
    Dp,
    Sk,
    Hw,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "start" => Cave::Start,
            "end" => Cave::End,
            "by" => Cave::By,
            "zs" => Cave::Zs,
            "zt" => Cave::Zt,
            "gv" => Cave::Gv,
            "iu" => Cave::Iu,
            "WO" => Cave::Wo,
            "QJ" => Cave::Qj,
            "DP" => Cave::Dp,
            "SK" => Cave::Sk,
            "HW" => Cave::Hw,
            _ => return Err(()),
        };

        Ok(out)
    }
}

const BY_1: u16 = 1 << 0;
const BY_2: u16 = 1 << 1;
const ZS_1: u16 = 1 << 2;
const ZS_2: u16 = 1 << 3;
const ZT_1: u16 = 1 << 4;
const ZT_2: u16 = 1 << 5;
const GV_1: u16 = 1 << 6;
const GV_2: u16 = 1 << 7;
const IU_1: u16 = 1 << 8;
const IU_2: u16 = 1 << 9;

const ANY_2_BUT_BY: u16 = ZS_2 | ZT_2 | GV_2 | IU_2;
const ANY_2_BUT_ZS: u16 = BY_2 | ZT_2 | GV_2 | IU_2;
const ANY_2_BUT_ZT: u16 = ZS_2 | BY_2 | GV_2 | IU_2;
const ANY_2_BUT_GV: u16 = ZS_2 | BY_2 | ZT_2 | IU_2;
const ANY_2_BUT_IU: u16 = ZS_2 | BY_2 | ZT_2 | GV_2;

#[derive(Clone, Debug)]
struct Path {
    last_node: Cave,
    counts: u16,
}

impl Path {
    fn count(&self, s: &Cave) -> u8 {
        match s {
            Cave::By => {
                if self.counts & BY_2 != 0 {
                    return 2;
                }
                if self.counts & BY_1 != 0 {
                    return 1;
                }

                0
            }
            Cave::Zs => {
                if self.counts & ZS_2 != 0 {
                    return 2;
                }
                if self.counts & ZS_1 != 0 {
                    return 1;
                }

                0
            }
            Cave::Zt => {
                if self.counts & ZT_2 != 0 {
                    return 2;
                }
                if self.counts & ZT_1 != 0 {
                    return 1;
                }

                0
            }
            Cave::Gv => {
                if self.counts & GV_2 != 0 {
                    return 2;
                }
                if self.counts & GV_1 != 0 {
                    return 1;
                }

                0
            }
            Cave::Iu => {
                if self.counts & IU_2 != 0 {
                    return 2;
                }
                if self.counts & IU_1 != 0 {
                    return 1;
                }

                0
            }
            other => unreachable!("{:?}", other),
        }
    }

    fn any_two_but(&self, s: &Cave) -> bool {
        match s {
            Cave::By => self.counts & ANY_2_BUT_BY != 0,
            Cave::Zs => self.counts & ANY_2_BUT_ZS != 0,
            Cave::Zt => self.counts & ANY_2_BUT_ZT != 0,
            Cave::Gv => self.counts & ANY_2_BUT_GV != 0,
            Cave::Iu => self.counts & ANY_2_BUT_IU != 0,
            other => unreachable!("{:?}", other),
        }
    }

    fn plus_one(&mut self, s: &Cave) {
        match s {
            Cave::By => match self.count(s) {
                0 => self.counts |= BY_1,
                1 => self.counts = (self.counts & !BY_1) | BY_2,
                _ => panic!(),
            },
            Cave::Zs => match self.count(s) {
                0 => self.counts |= ZS_1,
                1 => self.counts = (self.counts & !ZS_1) | ZS_2,
                _ => panic!(),
            },
            Cave::Zt => match self.count(s) {
                0 => self.counts |= ZT_1,
                1 => self.counts = (self.counts & !ZT_1) | ZT_2,
                _ => panic!(),
            },
            Cave::Gv => match self.count(s) {
                0 => self.counts |= GV_1,
                1 => self.counts = (self.counts & !GV_1) | GV_2,
                _ => panic!(),
            },
            Cave::Iu => match self.count(s) {
                0 => self.counts |= IU_1,
                1 => self.counts = (self.counts & !IU_1) | IU_2,
                _ => panic!(),
            },
            other => unreachable!("{:?}", other),
        }
    }
}

#[derive(Debug)]
struct CaveNetwork {
    adjacencies: HashMap<Cave, Vec<Cave>>,
}

const START_NODE: Cave = Cave::Start;
const END_NODE: Cave = Cave::End;

impl CaveNetwork {
    fn paths(&self) -> usize {
        let mut paths_count = 0;

        let mut traversal_stack = vec![];

        let start_path = Path {
            last_node: START_NODE,
            counts: 0,
        };

        traversal_stack.push(start_path);

        while let Some(current_path) = traversal_stack.pop() {
            if current_path.last_node == END_NODE {
                paths_count += 1;
                continue;
            }

            let out_nodes = self.adjacencies.get(&current_path.last_node).unwrap();

            for out_node in out_nodes {
                match out_node {
                    Cave::Start => continue,
                    Cave::End => {
                        let mut clonepath = current_path.clone();
                        clonepath.last_node = *out_node;
                        traversal_stack.push(clonepath);
                    }
                    // small caves
                    Cave::By | Cave::Zs | Cave::Zt | Cave::Gv | Cave::Iu => {
                        let others_have_two = current_path.any_two_but(out_node);

                        let this_visit_count = current_path.count(out_node);

                        if this_visit_count == 2 || (others_have_two && this_visit_count == 1) {
                            continue;
                        } else {
                            let mut clonepath = current_path.clone();
                            clonepath.plus_one(out_node);
                            clonepath.last_node = *out_node;
                            traversal_stack.push(clonepath);
                        }
                    }
                    // large caves
                    Cave::Wo | Cave::Qj | Cave::Dp | Cave::Sk | Cave::Hw => {
                        let mut clonepath = current_path.clone();
                        clonepath.last_node = *out_node;
                        traversal_stack.push(clonepath);
                    }
                }
            }
        }

        paths_count
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

    let paths = cave_network.paths();

    dbg!(std::mem::size_of::<Path>());
    dbg!(std::mem::size_of::<&str>());

    dbg!(paths);
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
        assert_eq!(paths, 120506)
    }
}
