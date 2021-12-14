use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;
use std::vec::Vec;

fn get_input() -> Vec<i32> {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut line = String::new();
    buf_reader.read_line(&mut line);
    line.split(",")
        .filter_map(|num| num.to_string().parse::<i32>().ok())
        .collect()
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut min_alignment = i32::MAX;
    for position in input {
        min_alignment = std::cmp::min(
            min_alignment,
            input
                .iter()
                .fold(0, |total, p| total + (p - position).abs()),
        )
    }
    min_alignment
}

fn part2(input: &Vec<i32>) -> i32 {
    let min_position = 0;
    let max_position = *input.iter().max().unwrap();

    let mut min_alignment = i32::MAX;

    let triangle_number = |n: i32| -> i32 { (n * (n + 1)) / 2 };

    for position in min_position..=max_position {
        min_alignment = std::cmp::min(
            min_alignment,
            input
                .iter()
                .fold(0, |total, p| total + triangle_number((p - position).abs())),
        )
    }
    min_alignment
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
