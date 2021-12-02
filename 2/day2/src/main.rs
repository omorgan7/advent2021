use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::vec::Vec;

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32)
}

fn get_input() -> Vec<Direction> {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
  

    contents.split("\n").map(|line| {
      let mut split = line.split(" ");
      match split.next().unwrap() {
          "forward" => Direction::Forward(split.last().unwrap().parse::<i32>().unwrap()),
          "down" => Direction::Down(split.last().unwrap().parse::<i32>().unwrap()),
          "up" => Direction::Up(split.last().unwrap().parse::<i32>().unwrap()),
          _ => panic!()
      }
    }).collect()
}

fn part1(input : &Vec<Direction>) -> i32 {
    let position = input.iter().fold((0, 0), |position, d| {
        match d {
            Direction::Forward(x) => (position.0 + x, position.1),
            Direction::Up(y) => (position.0, position.1 - y),
            Direction::Down(y) => (position.0, position.1 + y)
        }
    });

    position.0 * position.1
}

fn part2(input : &Vec<Direction>) -> i32 {
    let position = input.iter().fold((0, 0, 0), |position, d| {
        match d {
            Direction::Forward(x) => (position.0 + x, position.1 + x * position.2, position.2),
            Direction::Up(y) => (position.0, position.1, position.2 - y),
            Direction::Down(y) => (position.0, position.1, position.2 + y)
        }
    });

    position.0 * position.1
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
