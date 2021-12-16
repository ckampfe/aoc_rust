use std::io::BufRead;

#[derive(Clone, Debug, PartialEq)]
enum Packet {
    Literal {
        header: Header,
        value: usize,
    },
    Operator {
        header: Header,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn score(&self) -> usize {
        let mut s = 0;
        match self {
            Packet::Literal { header, .. } => s += header.version,
            Packet::Operator { header, packets } => {
                s += header.version;

                for packet in packets {
                    s += packet.score();
                }
            }
        }
        s
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Header {
    version: usize,
}

fn parse_packet(s: &str) -> (&str, Packet) {
    dbg!("enter parse pakcket");
    let (version, s) = take(s, 3);
    let (type_id, s) = take(s, 3);
    let version = binary_to_decimal(version);
    let type_id = binary_to_decimal(type_id);

    let header = match type_id {
        // literal
        4 => Header { version },
        // operator
        _ => Header { version },
    };

    match type_id {
        4 => {
            let (s, value) = parse_literal(s);
            (s, Packet::Literal { header, value })
        }
        _ => {
            let (s, packets) = parse_operator(s);
            (s, Packet::Operator { header, packets })
        }
    }
}

fn parse_literal(s: &str) -> (&str, usize) {
    dbg!("enter parse literal");
    let mut n = String::new();
    let mut loop_s = s;
    loop {
        let (bit, inner_s) = take(loop_s, 1);
        let (four_bits, inner_s) = take(inner_s, 4);
        loop_s = inner_s;
        n.push_str(four_bits);
        if bit == "0" {
            return (loop_s, binary_to_decimal(&n));
        }
    }
}

fn parse_operator(s: &str) -> (&str, Vec<Packet>) {
    dbg!("enter parse operator");
    let (length_type_id, s) = take(s, 1);
    dbg!(length_type_id);

    let mut packets = vec![];

    match length_type_id {
        "0" => {
            let (total_length_in_bits, s) = take(s, 15);
            dbg!(total_length_in_bits);
            let total_length_in_bits = binary_to_decimal(total_length_in_bits);
            dbg!(total_length_in_bits);
            dbg!(s.len());
            let (sub_packets_bits, s) = take(s, total_length_in_bits);

            let mut spb = sub_packets_bits;

            loop {
                if spb.is_empty() {
                    return (s, packets);
                } else {
                    let (remaining, packet) = parse_packet(spb);
                    spb = remaining;
                    packets.push(packet);
                }
            }
        }
        "1" => {
            let (number_of_sub_packets, s) = take(s, 11);
            let number_of_sub_packets = binary_to_decimal(number_of_sub_packets);
            dbg!(&number_of_sub_packets);
            let mut spb = s;
            for i in 0..number_of_sub_packets {
                dbg!(i);
                let (remaining, packet) = parse_packet(spb);
                dbg!(&packet);
                spb = remaining;
                packets.push(packet);
            }

            (spb, packets)
        }
        bad => panic!("Bad length_type_id bit reached in parse_operator: {}", bad),
    }
}

fn take(s: &str, n: usize) -> (&str, &str) {
    s.split_at(n)
}

fn hex_to_bin(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!(),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn binary_to_decimal(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn main() {
    let input = std::fs::File::open("inputs/16.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());

    for line in file_lines {
        let bin = hex_to_bin(&line);
        let (_, packet) = parse_packet(&bin);
        dbg!(packet.score());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {
        let hex = "D2FE28";
        let as_bin = hex_to_bin(hex);
        let (_s, packet) = parse_packet(&as_bin);
        assert_eq!(
            packet,
            Packet::Literal {
                header: Header { version: 6 },
                value: 2021
            }
        )
    }

    #[test]
    fn operator_zero() {
        let hex = "38006F45291200";
        let as_bin = hex_to_bin(hex);
        let (_s, packet) = parse_packet(&as_bin);
        assert_eq!(
            packet,
            Packet::Operator {
                header: Header { version: 1 },
                packets: vec![
                    Packet::Literal {
                        header: Header { version: 6 },
                        value: 10
                    },
                    Packet::Literal {
                        header: Header { version: 2 },
                        value: 20
                    }
                ]
            }
        )
    }

    #[test]
    fn operator_one() {
        let hex = "EE00D40C823060";
        let as_bin = hex_to_bin(hex);
        let (_s, packet) = parse_packet(&as_bin);
        assert_eq!(
            packet,
            Packet::Operator {
                header: Header { version: 7 },
                packets: vec![
                    Packet::Literal {
                        header: Header { version: 2 },
                        value: 1
                    },
                    Packet::Literal {
                        header: Header { version: 4 },
                        value: 2
                    },
                    Packet::Literal {
                        header: Header { version: 1 },
                        value: 3
                    }
                ]
            }
        )
    }
}
