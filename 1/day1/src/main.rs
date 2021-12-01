use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;

fn get_input() -> Vec<i32> {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
  

    contents.split("\n").map(|line| {
      line.parse::<i32>().unwrap()
    }).collect()
}

fn part1(input : &Vec<i32>) -> i32 {

    let mut prev = input[0].clone();
    input.iter().fold(0, |total, depth| {
        let new_total = total + (depth > &prev) as i32;
        prev = *depth;
        new_total
    })
}

fn part2(input : &Vec<i32>) -> i32 {

    let mut prev_total = input[0] + input[1] + input[2];
    input[3..].iter().zip(input.iter()).fold(0, |total, depth| {
        let new_depth_total = prev_total - depth.1 + depth.0;
        let new_total = total + (new_depth_total > prev_total) as i32;
        prev_total = new_depth_total;
        new_total
    })
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
