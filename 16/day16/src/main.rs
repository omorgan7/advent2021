use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

type Input = Vec<char>;
type Binop = fn(i64, i64) -> i64;

#[derive(Debug, Clone)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}
#[derive(Debug, Clone)]
struct Literal {
    version: i64,
    value: i64,
}
#[derive(Debug, Clone)]
struct Operator {
    version: i64,
    op: Binop,
    subpackets: Vec<Packet>,
}

fn expand(c: char) -> &'static str {
    match c {
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
    }
}

fn from_binary(binary: &[char]) -> i64 {
    let mut multiplier = 1;

    binary.iter().rev().fold(0, |total, c| {
        let new_total = total + (*c as i64 - '0' as i64) * multiplier;
        multiplier *= 2;
        new_total
    })
}

fn get_input() -> Input {
    let buf_reader = BufReader::new(File::open("input.txt").unwrap());
    buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
}

fn process_literal(binary: &[char], version: i64) -> (&[char], Literal) {
    let mut offset = 0;
    let mut numbers = Vec::<i64>::new();

    loop {
        let keep_going = from_binary(&binary[offset..offset + 1]);
        numbers.push(from_binary(&binary[offset + 1..offset + 5]));

        offset += 5;

        if keep_going == 0 {
            break;
        }

        if binary.len() < offset {
            break;
        }
    }

    let mut multiplier = 1;
    let value = numbers.iter().rev().fold(0, |total, x| {
        let new_total = x * multiplier + total;
        multiplier *= 16;
        new_total
    });

    (
        &binary[offset..],
        Literal {
            version: version,
            value: value,
        },
    )
}

fn process(binary: &[char]) -> (&[char], Packet) {
    let version = from_binary(&binary[0..3]);
    let type_id = from_binary(&binary[3..6]);
    return match type_id {
        4 => {
            let (range, literal) = process_literal(&binary[6..], version);
            (range, Packet::Literal(literal))
        }
        _ => {
            let op: Binop = match type_id {
                0 => |a, b| a + b,
                1 => |a, b| a * b,
                2 => |a, b| if a < b { a } else { b },
                3 => |a, b| if a > b { a } else { b },
                5 => |a, b| if a > b { 1 } else { 0 },
                6 => |a, b| if a < b { 1 } else { 0 },
                7 => |a, b| if a == b { 1 } else { 0 },
                _ => panic!(),
            };

            let mut operator_packet = Operator {
                version: version,
                op: op,
                subpackets: Vec::<Packet>::new(),
            };

            let length_type_id = from_binary(&binary[6..7]);
            let skip_length = if length_type_id == 0 { 15 } else { 11 };

            if skip_length == 15 {
                let bit_count = from_binary(&binary[7..7 + skip_length]);

                let mut r = &binary[7 + skip_length..];
                let upper_bound = r.len();
                loop {
                    let (range, packet) = process(r);
                    r = range;

                    operator_packet.subpackets.push(packet);

                    if upper_bound - r.len() == bit_count as usize {
                        break;
                    }
                }

                (r, Packet::Operator(operator_packet))
            } else {
                let count = from_binary(&binary[7..7 + skip_length]);

                let mut r = &binary[7 + skip_length as usize..];
                for _ in 0..count {
                    let (range, packet) = process(r);
                    r = range;
                    operator_packet.subpackets.push(packet);
                }

                (r, Packet::Operator(operator_packet))
            }
        }
    };
}

struct Version {
    v: i64,
}

impl Version {
    fn process(&mut self, packet: &Packet) {
        match packet {
            Packet::Literal(x) => self.v += x.version,
            Packet::Operator(x) => self.v += x.version,
        }
    }
}

fn process_packet_1(packet: &Packet, cb: &mut Version) {
    match packet {
        Packet::Literal(_) => {
            cb.process(&packet);
        }
        Packet::Operator(operator) => {
            cb.process(&packet);
            operator
                .subpackets
                .iter()
                .for_each(|p| process_packet_1(&p, cb));
        }
    }
}

fn process_packet_2(packet: &Packet) -> i64 {
    match packet {
        Packet::Literal(literal) => literal.value,
        Packet::Operator(operator) => operator.subpackets[1..]
            .iter()
            .fold(process_packet_2(&operator.subpackets[0]), |acc, p| {
                (operator.op)(acc, process_packet_2(p))
            }),
    }
}

fn part1(input: &Input) -> i64 {
    let binary = input
        .iter()
        .map(|c| expand(*c).to_string().chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    let mut version_sum = Version { v: 0 };

    let outer = process(&binary[..]).1;

    process_packet_1(&outer, &mut version_sum);
    version_sum.v
}

fn part2(input: &Input) -> i64 {
    let binary = input
        .iter()
        .map(|c| expand(*c).to_string().chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    let outer = process(&binary[..]).1;
    process_packet_2(&outer)
}

fn main() {
    let input = get_input();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
