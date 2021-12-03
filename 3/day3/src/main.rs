use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;

fn get_input() -> Vec<String> {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
  

    contents.split("\n").map(|line| line.to_string()).collect()
}

fn part1(input : &Vec<String>) -> u64 {
    let number_length = input[0].len();

    let half_total = input.len() / 2;
    let one_totals : Vec<usize> = (0..number_length).into_iter().map(|index| {
        input.iter().fold(0, |total, line| {
            total + (line.as_bytes()[index] as char == '1') as usize
        })
    }).collect(); 

    let gamma = 
        u64::from_str_radix(&one_totals
            .iter()
            .map(|count| {
                if count > &half_total { '1' } else { '0' }
            }).collect::<String>(), 2).unwrap();
    let epislon = !(gamma) & 0b111111111111u64;

    gamma * epislon
}

fn part2(input : &Vec<String>) -> u64 {

    let mut oxygen = input.clone();
    let mut carbon = input.clone();

    for index in (0..input[0].len()) {

        let filter = | set : Vec<String>, zero : char, one : char | {
            let half_total =
                if set.len() % 2 == 0 {
                    (set.len() / 2 ) as u64
                }
                else {
                    ((set.len() + 1) / 2 ) as u64
                };

            let one_totals = set.iter().fold(0, |total, line| {
                total + (line.as_bytes()[index] as char == '1') as u64
            });

            let select = if set.len() == 1 { set[0].as_bytes()[index] as char } else if one_totals >= half_total { zero } else { one };

            set.iter().filter(|line| {
                line.as_bytes()[index] as char == select
            }).cloned().collect()
        };

        carbon = filter(carbon, '0', '1');
        oxygen = filter(oxygen, '1', '0');
    }

    u64::from_str_radix(&oxygen[0], 2).unwrap() * u64::from_str_radix(&carbon[0], 2).unwrap()
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
