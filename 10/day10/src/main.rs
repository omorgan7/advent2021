use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

fn get_input() -> Vec<Vec<char>> {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    buf_reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn bracket_to_opposite(b: char) -> char {
    match b {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',

        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!(),
    }
}

fn part1(input: &Vec<Vec<char>>) -> i32 {
    input.iter().fold(0, |score_total, line| {
        let mut stack = Vec::<char>::new();

        let mut found = '\0';
        for c in line {
            match c {
                '(' | '{' | '[' | '<' => {
                    stack.push(*c);
                }
                ')' | '}' | ']' | '>' => {
                    let opposite = stack.pop().unwrap();
                    if opposite != bracket_to_opposite(*c) {
                        found = *c;
                        break;
                    }
                }
                _ => (),
            }
        }

        let bracket_to_score = |b: char| -> i32 {
            match b {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!(),
            }
        };

        if found != '\0' {
            return score_total + bracket_to_score(found);
        } else {
            return score_total;
        }
    })
}

fn part2(input: &Vec<Vec<char>>) -> i64 {
    let mut scores = input
        .iter()
        .filter_map(|line| {
            let mut stack = Vec::<char>::new();

            for c in line {
                match c {
                    '(' | '{' | '[' | '<' => {
                        stack.push(*c);
                    }
                    ')' | '}' | ']' | '>' => {
                        let opposite = stack.pop().unwrap();
                        if opposite != bracket_to_opposite(*c) {
                            return None;
                        }
                    }
                    _ => (),
                }
            }

            let mut remainder = Vec::<char>::new();
            while stack.is_empty() == false {
                let opposite = stack.pop().unwrap();
                remainder.push(bracket_to_opposite(opposite));
            }

            let bracket_to_score = |b: char| -> i64 {
                match b {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!(),
                }
            };

            Some(
                remainder
                    .iter()
                    .fold(0i64, |total, r| 5 * total + bracket_to_score(*r)),
            )
        })
        .collect::<Vec<i64>>();

    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
