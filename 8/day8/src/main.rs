use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;
use std::vec::Vec;

#[derive(Debug)]
struct Input {
    patterns: Vec<Vec<String>>,
    digits: Vec<Vec<String>>,
}

fn get_input() -> Input {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|s| s.ok())
        .collect::<Vec<String>>();

    let patterns = lines
        .iter()
        .filter_map(|line| {
            let mut parts = line.split(" | ");
            Some(
                parts
                    .nth(0)?
                    .split(" ")
                    .map(|word| String::from(word))
                    .collect(),
            )
        })
        .collect();

    let digits = lines
        .iter()
        .filter_map(|line| {
            let mut parts = line.split(" | ");
            Some(
                parts
                    .nth(1)?
                    .split(" ")
                    .map(|word| String::from(word))
                    .collect(),
            )
        })
        .collect();

    Input {
        patterns: patterns,
        digits: digits,
    }
}

fn part1(input: &Input) -> i32 {
    input.digits.iter().fold(0, |total, digits| {
        total
            + digits.iter().fold(0, |t, d| {
                t + (d.as_bytes().len() == 2
                    || d.as_bytes().len() == 4
                    || d.as_bytes().len() == 3
                    || d.as_bytes().len() == 7) as i32
            })
    })
}

fn difference(a: &String, b: &String) -> String {
    let mut string = String::new();

    for c in a.as_bytes().iter() {
        match b.as_bytes().iter().find(|x| *x == c) {
            None => string.push(*c as char),
            _ => (),
        }
    }

    for c in b.as_bytes().iter() {
        match a.as_bytes().iter().find(|x| *x == c) {
            None => string.push(*c as char),
            _ => (),
        }
    }

    string
}

fn subtract(a: &String, b: &String) -> Option<String> {
    if b.as_bytes().len() > a.as_bytes().len() {
        return None;
    }

    if difference(&a, &b) == *a {
        return None;
    }

    let mut result = String::new();

    for aa in a.as_bytes().iter() {
        if b.as_bytes().iter().find(|x| *x == aa).is_none() {
            result.push(*aa as char);
        }
    }

    if result.as_bytes().len() == (a.as_bytes().len() - b.as_bytes().len()) {
        Some(result)
    } else {
        None
    }
}

fn intersection(a: &String, b: &String) -> String {
    let mut string = String::new();

    for c in a.as_bytes().iter() {
        match b.as_bytes().iter().find(|x| *x == c) {
            Some(_) => string.push(*c as char),
            _ => (),
        }
    }

    string
}

fn part2(input: &Input) -> i32 {
    input
        .patterns
        .iter()
        .zip(input.digits.iter())
        .fold(0, |total, (patterns, digits)| {
            let one_digits = patterns.iter().find(|x| x.as_bytes().len() == 2).unwrap();
            let seven_digits = patterns.iter().find(|x| x.as_bytes().len() == 3).unwrap();
            let four_digits = patterns.iter().find(|x| x.as_bytes().len() == 4).unwrap();
            let eight_digits = patterns.iter().find(|x| x.as_bytes().len() == 7).unwrap();

            // the base cases that can be computed from the unique digits:

            let mut solved_digits = [0xFF as char; 7];

            // |
            // |
            let top_bottom_right = one_digits;

            solved_digits[0] = difference(one_digits, seven_digits).as_bytes()[0] as char;

            // |
            //  _
            let bottom_left = difference(
                &(one_digits.to_owned() + seven_digits + four_digits),
                eight_digits,
            );

            // |
            //  -
            //
            let top_left_middle = difference(one_digits, four_digits);

            // search for a string that contains
            // either the TBR, BR or TLM strings
            // but only 1 of them, and importantly contains all others

            let mut two_solved = false;
            let mut five_solved = false;

            while solved_digits.iter().any(|d| *d == 0xFF as char) {
                for p in patterns.iter().filter(|p| p.as_bytes().len() == 5) {
                    let maybe_subtract = subtract(&p, &String::from(solved_digits[0]));
                    if maybe_subtract.is_none() {
                        continue;
                    }

                    let mut test = maybe_subtract.unwrap();
                    for d in &solved_digits[1..] {
                        if *d != (0xFF as char) {
                            let tmp = subtract(&test, &String::from(*d));
                            if tmp.is_none() {
                                continue;
                            }
                            test = tmp.unwrap();
                        }
                    }

                    if two_solved == false {
                        let maybe_two = subtract(&test, &bottom_left);
                        if maybe_two.is_none() {
                            continue;
                        }
                        let two = maybe_two.unwrap();

                        let middle = intersection(&two, &top_left_middle);
                        solved_digits[3] = middle.as_bytes()[0] as char;
                        solved_digits[2] =
                            subtract(&top_left_middle, &middle).unwrap().as_bytes()[0] as char;
                        solved_digits[1] = subtract(&two, &middle).unwrap().as_bytes()[0] as char;
                        solved_digits[4] =
                            subtract(&top_bottom_right, &String::from(solved_digits[1]))
                                .unwrap()
                                .as_bytes()[0] as char;

                        two_solved = true;
                        continue;
                    }

                    if five_solved == false {
                        if test.as_bytes().len() != 1 {
                            continue;
                        }

                        solved_digits[6] = test.as_bytes()[0] as char;
                        solved_digits[5] =
                            subtract(&bottom_left, &test).unwrap().as_bytes()[0] as char;

                        five_solved = true;
                        continue;
                    }
                }
            }

            let decode: Vec<String> = {
                let mut tmp: [String; 10] = [
                    // 0
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(i, d)| if i != 3 { Some(d) } else { None })
                        .collect(),
                    // 1
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(i, d)| if i == 1 || i == 4 { Some(d) } else { None })
                        .collect(),
                    // 2
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(i, d)| if i != 2 && i != 4 { Some(d) } else { None })
                        .collect(),
                    // 3
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(i, d)| if i != 2 && i != 5 { Some(d) } else { None })
                        .collect(),
                    // 4
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(i, d)| {
                            if i != 0 && i != 5 && i != 6 {
                                Some(d)
                            } else {
                                None
                            }
                        })
                        .collect(),
                    // 5
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(i, d)| if i != 1 && i != 5 { Some(d) } else { None })
                        .collect(),
                    // 6
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(i, d)| if i != 1 { Some(d) } else { None })
                        .collect(),
                    // 7
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(i, d)| {
                            if i == 1 || i == 4 || i == 0 {
                                Some(d)
                            } else {
                                None
                            }
                        })
                        .collect(),
                    // 8
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(_, d)| Some(d))
                        .collect(),
                    // 9
                    solved_digits
                        .iter()
                        .enumerate()
                        .filter_map(|(i, d)| if i != 5 { Some(d) } else { None })
                        .collect(),
                ];
                for d in &mut tmp {
                    let t0 = &d[..];
                    let mut chars: Vec<char> = t0.chars().collect();
                    chars.sort_by(|a, b| b.cmp(a));

                    *d = chars.iter().collect::<String>();
                }
                tmp.to_vec()
            };

            let mut sum = 0i32;
            for (index, digit) in digits.iter().rev().enumerate() {
                let decoded = decode
                    .iter()
                    .enumerate()
                    .find(|(_, d)| {
                        let mut t0 = &digit[..];
                        let mut chars: Vec<char> = t0.chars().collect();
                        chars.sort_by(|a, b| b.cmp(a));

                        let tmp = chars.iter().collect::<String>();
                        **d == tmp
                    })
                    .unwrap()
                    .0;

                sum += decoded as i32 * (10i32).pow(index as u32);
            }

            total + sum
        })
}

fn main() {
    let input = get_input();
    println!("{:?}", input);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
