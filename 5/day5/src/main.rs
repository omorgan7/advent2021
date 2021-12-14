use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

extern crate num;
use num::range_step;

#[derive(Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

struct Input {
    start: Vec<Vec2>,
    end: Vec<Vec2>,
}

fn is_horizontal(start: &Vec2, end: &Vec2) -> bool {
    start.y == end.y
}

fn is_vertical(start: &Vec2, end: &Vec2) -> bool {
    start.x == end.x
}

fn get_input() -> Input {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let to_vec2 = |num: &str| -> Vec2 {
        let num_str = num.to_string().replace("\n", "");
        let mut numbers = num_str.split(",");
        let x = numbers.next().unwrap().parse::<i32>().unwrap();
        let y = numbers.next().unwrap().parse::<i32>().unwrap();

        Vec2 { x: x, y: y }
    };

    let starts_ends = buf_reader
        .lines()
        .map(|line| {
            let line_ok = line.unwrap();
            (
                to_vec2(line_ok.split(" -> ").next().unwrap()),
                to_vec2(line_ok.split(" -> ").skip(1).next().unwrap()),
            )
        })
        .collect::<Vec<(Vec2, Vec2)>>();

    Input {
        start: starts_ends.iter().map(|x| x.0).collect(),
        end: starts_ends.iter().map(|x| x.1).collect(),
    }
}

fn part1(input: &Input) -> i32 {
    let width = input
        .start
        .iter()
        .zip(input.end.iter())
        .fold(0, |max, elem| {
            std::cmp::max(max, std::cmp::max(elem.0.x, elem.1.x))
        })
        + 1;

    let height = input
        .start
        .iter()
        .zip(input.end.iter())
        .fold(0, |max, elem| {
            std::cmp::max(max, std::cmp::max(elem.0.y, elem.1.y))
        })
        + 1;

    let mut space = Vec::<i32>::new();
    space.resize(width as usize * height as usize, 0);

    for elem in input.start.iter().zip(input.end.iter()) {
        let start = elem.0;
        let end = elem.1;
        if is_horizontal(&start, &end) {
            let x_start = if start.x > end.x { end.x } else { start.x };
            let x_end = if start.x > end.x { start.x } else { end.x };
            for x in x_start..=x_end {
                space[(start.y * width + x) as usize] += 1
            }
        }

        if is_vertical(&start, &end) {
            let y_start = if start.y > end.y { end.y } else { start.y };
            let y_end = if start.y > end.y { start.y } else { end.y };
            for y in y_start..=y_end {
                space[(width * y + start.x) as usize] += 1
            }
        }
    }

    space.iter().fold(0, |total, s| total + (*s > 1) as i32)
}

fn part2(input: &Input) -> i32 {
    let width = input
        .start
        .iter()
        .zip(input.end.iter())
        .fold(0, |max, elem| {
            std::cmp::max(max, std::cmp::max(elem.0.x, elem.1.x))
        })
        + 1;

    let height = input
        .start
        .iter()
        .zip(input.end.iter())
        .fold(0, |max, elem| {
            std::cmp::max(max, std::cmp::max(elem.0.y, elem.1.y))
        })
        + 1;

    let mut space = Vec::<i32>::new();
    space.resize(width as usize * height as usize, 0);

    let is_diagonal = |start: &Vec2, end: &Vec2| {
        let x_grad = end.x - start.x;
        let y_grad = end.y - start.y;

        x_grad == y_grad || x_grad == -y_grad
    };

    for elem in input.start.iter().zip(input.end.iter()) {
        let start = elem.0;
        let end = elem.1;

        let x_start = if start.x > end.x { end.x } else { start.x };
        let x_end = if start.x > end.x { start.x } else { end.x };

        let y_start = if start.y > end.y { end.y } else { start.y };
        let y_end = if start.y > end.y { start.y } else { end.y };

        if is_horizontal(&start, &end) {
            for x in x_start..=x_end {
                space[(start.y * width + x) as usize] += 1
            }
        }

        if is_vertical(&start, &end) {
            for y in y_start..=y_end {
                space[(width * y + start.x) as usize] += 1
            }
        }

        if is_diagonal(&start, &end) {
            let x_direction = if start.x < end.x { 1 } else { -1 };
            let y_direction = if start.y < end.y { 1 } else { -1 };

            let end_x = if start.x < end.x {
                end.x + 1
            } else {
                end.x - 1
            };
            let end_y = if start.y < end.y {
                end.y + 1
            } else {
                end.y - 1
            };
            for (x, y) in
                range_step(start.x, end_x, x_direction).zip(range_step(start.y, end_y, y_direction))
            {
                space[(width * y + x) as usize] += 1
            }
        }
    }

    space.iter().fold(0, |total, s| total + (*s > 1) as i32)
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
