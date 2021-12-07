use std::fs::File;
use std::io::BufReader;
use std::string::String;
use std::io::prelude::*;
use std::io::SeekFrom::Current;
use std::vec::Vec;


fn get_input() -> Vec::<i8> {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut line = String::new();
    buf_reader.read_line(&mut line);
    line.split(",").filter_map(|num| {
        num.to_string().parse::<i8>().ok()
    }).collect()
}

fn part1(input : &Vec::<i8>, target : i32) -> i64 {

    let mut fish = input.clone();
    let mut new_fish = 0;
    for it in 0..target {
        
        let prev_len = fish.len();
        for x in 0..new_fish {
            fish.push(8);
        }
        new_fish = 0;

        for (index, x) in fish.iter_mut().enumerate().take(prev_len) {
            if *x == 0 {
                *x = 6;
            }
            else {
                *x -= 1;
                if *x == 0 {
                    new_fish += 1;
                }
            }
        }
    }
    fish.len() as i64
}

fn part2(input : &Vec::<i8>, target : i32) -> i64 {

    let mut fish_count = [0i64; 9];
    let mut fish_count_prev = [0i64; 9];

    for x in input {
        fish_count[*x as usize] += 1;
    }

    for _ in 0..target {
        fish_count_prev = fish_count;
        fish_count[8] = fish_count_prev[0];
        
        for i in 1..9 {
            if i == 7 {
                fish_count[6] = fish_count_prev[7] + fish_count_prev[0];
            }
            else {
                fish_count[i - 1] = fish_count_prev[i];
            }
        }
    }
    
    fish_count.iter().sum::<i64>() as i64
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input, 80));
    println!("{}", part2(&input, 256));
}
