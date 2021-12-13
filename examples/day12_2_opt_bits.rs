use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

type Edge = [u16; 2];

fn lex(s: &str) -> Edge {
    let caves_strs = s.split('-');

    let mut edge = [0; 2];

    for (i, cave) in caves_strs.enumerate() {
        let c = Cave::from_str(cave).unwrap();
        edge[i] = u16::from(c);
    }

    edge
}

const BLANK_CAVE: u16 = 0b000000_11_1111_1111;

const START: u16 = 0b001100_00_0000_0000;
const END: u16 = 0b000001_00_0000_0000;
const BY: u16 = 0b000010_00_0000_0000;
const ZS: u16 = 0b000011_00_0000_0000;
const ZT: u16 = 0b000100_00_0000_0000;
const GV: u16 = 0b000101_00_0000_0000;
const IU: u16 = 0b000110_00_0000_0000;
const WO: u16 = 0b000111_00_0000_0000;
const QJ: u16 = 0b001000_00_0000_0000;
const DP: u16 = 0b001001_00_0000_0000;
const SK: u16 = 0b001010_00_0000_0000;
const HW: u16 = 0b001011_00_0000_0000;

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

impl From<Cave> for u16 {
    fn from(cave: Cave) -> Self {
        match cave {
            Cave::Start => START,
            Cave::End => END,
            Cave::By => BY,
            Cave::Zs => ZS,
            Cave::Zt => ZT,
            Cave::Gv => GV,
            Cave::Iu => IU,
            Cave::Wo => WO,
            Cave::Qj => QJ,
            Cave::Dp => DP,
            Cave::Sk => SK,
            Cave::Hw => HW,
        }
    }
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
    repr: u16,
}

impl Path {
    fn count(&self, s: u16) -> u8 {
        let as_node = s & !BLANK_CAVE;

        match as_node {
            BY => {
                if self.repr & BY_2 != 0 {
                    return 2;
                }
                if self.repr & BY_1 != 0 {
                    return 1;
                }

                0
            }
            ZS => {
                if self.repr & ZS_2 != 0 {
                    return 2;
                }
                if self.repr & ZS_1 != 0 {
                    return 1;
                }

                0
            }
            ZT => {
                if self.repr & ZT_2 != 0 {
                    return 2;
                }
                if self.repr & ZT_1 != 0 {
                    return 1;
                }

                0
            }
            GV => {
                if self.repr & GV_2 != 0 {
                    return 2;
                }
                if self.repr & GV_1 != 0 {
                    return 1;
                }

                0
            }
            IU => {
                if self.repr & IU_2 != 0 {
                    return 2;
                }
                if self.repr & IU_1 != 0 {
                    return 1;
                }

                0
            }
            other => unreachable!("{:?}", other),
        }
    }

    fn any_two_but(&self, s: u16) -> bool {
        let as_node = s & !BLANK_CAVE;

        match as_node {
            BY => self.repr & ANY_2_BUT_BY != 0,
            ZS => self.repr & ANY_2_BUT_ZS != 0,
            ZT => self.repr & ANY_2_BUT_ZT != 0,
            GV => self.repr & ANY_2_BUT_GV != 0,
            IU => self.repr & ANY_2_BUT_IU != 0,
            other => unreachable!("{:?}", other),
        }
    }

    fn plus_one(&mut self, s: u16) {
        let as_node = s & !BLANK_CAVE;

        match as_node {
            BY => match self.count(s) {
                0 => self.repr |= BY_1,
                1 => self.repr = (self.repr & !BY_1) | BY_2,
                _ => panic!(),
            },
            ZS => match self.count(s) {
                0 => self.repr |= ZS_1,
                1 => self.repr = (self.repr & !ZS_1) | ZS_2,
                _ => panic!(),
            },
            ZT => match self.count(s) {
                0 => self.repr |= ZT_1,
                1 => self.repr = (self.repr & !ZT_1) | ZT_2,
                _ => panic!(),
            },
            GV => match self.count(s) {
                0 => self.repr |= GV_1,
                1 => self.repr = (self.repr & !GV_1) | GV_2,
                _ => panic!(),
            },
            IU => match self.count(s) {
                0 => self.repr |= IU_1,
                1 => self.repr = (self.repr & !IU_1) | IU_2,
                _ => panic!(),
            },
            other => unreachable!("{:?}", other),
        }
    }
}

#[derive(Debug)]
struct CaveNetwork {
    adjacencies: HashMap<u16, Vec<u16>>,
}

const START_NODE: u16 = START;
const END_NODE: u16 = END;

impl CaveNetwork {
    fn paths(&self) -> usize {
        let mut paths_count = 0;

        let mut traversal_stack = vec![];

        let start_path = Path { repr: START_NODE };

        traversal_stack.push(start_path);

        while let Some(current_path) = traversal_stack.pop() {
            let as_node = current_path.repr & !BLANK_CAVE;

            if as_node == END_NODE {
                paths_count += 1;
                continue;
            }

            let out_nodes = self.adjacencies.get(&as_node).unwrap();

            for out_node in out_nodes {
                match out_node {
                    _ if *out_node == START => continue,
                    _ if *out_node == END => {
                        let mut clonepath = current_path.clone();
                        clonepath.repr &= BLANK_CAVE;
                        clonepath.repr |= END;
                        traversal_stack.push(clonepath);
                    }
                    // small caves
                    // Cave::By | Cave::Zs | Cave::Zt | Cave::Gv | Cave::Iu
                    _ if BY == *out_node
                        || ZS == *out_node
                        || ZT == *out_node
                        || GV == *out_node
                        || IU == *out_node =>
                    {
                        let others_have_two = current_path.any_two_but(*out_node);

                        let this_visit_count = current_path.count(*out_node);

                        if this_visit_count == 2 || (others_have_two && this_visit_count == 1) {
                            continue;
                        } else {
                            let mut clonepath = current_path.clone();
                            clonepath.plus_one(*out_node);
                            clonepath.repr &= BLANK_CAVE;
                            clonepath.repr |= out_node;
                            traversal_stack.push(clonepath);
                        }
                    }
                    // large caves
                    // Cave::Wo | Cave::Qj | Cave::Dp | Cave::Sk | Cave::Hw => {
                    _ if WO == *out_node
                        || QJ == *out_node
                        || DP == *out_node
                        || SK == *out_node
                        || HW == *out_node =>
                    {
                        let mut clonepath = current_path.clone();
                        clonepath.repr &= BLANK_CAVE;
                        clonepath.repr |= out_node;
                        traversal_stack.push(clonepath);
                    }
                    other => panic!("{}", other),
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
