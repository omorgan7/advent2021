use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Debug)]
struct Input {
    width: usize,
    height: usize,
    grid: Vec<u8>,
}

fn get_input() -> Input {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut height = 0usize;
    let grid = buf_reader
        .lines()
        .flat_map(|line| -> Vec<u8> {
            height += 1;
            line.unwrap()
                .chars()
                .filter_map(|x| {
                    let tmp = x.to_digit(10);
                    if tmp.is_some() {
                        Some(tmp.unwrap() as u8)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect::<Vec<u8>>();

    let width = grid.len() / height;

    Input {
        width,
        height,
        grid,
    }
}

struct Sim {
    width: usize,
    height: usize,
    grid: Vec<u8>,
}

impl Sim {
    fn new(width: usize, height: usize, grid: Vec<u8>) -> Sim {
        Sim {
            width,
            height,
            grid,
        }
    }

    fn to_index(&self, x: i32, y: i32) -> usize {
        x as usize + y as usize * self.width
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        !(x < 0 || y < 0 || x >= (self.width as i32) || y >= (self.height as i32))
    }

    fn update(&mut self, x: i32, y: i32, mut already_flashed: &mut Vec<usize>) {
        if !self.in_bounds(x, y) {
            return;
        }

        let neighbours = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let i = self.to_index(x, y);
        self.grid[i] += 1;
        if self.grid[i] > 9 && already_flashed.iter().find(|x| **x == i).is_none() {
            already_flashed.push(i);
            for (dx, dy) in neighbours.iter() {
                self.update(x + dx, y + dy, &mut already_flashed);
            }
        }
    }
}

fn part1(input: &Input, steps: i32) -> i32 {
    let mut simulation = Sim::new(input.width, input.height, input.grid.clone());

    let mut flashes = 0i32;

    for _ in 0..steps {
        let mut already_flashed = Vec::<usize>::new();

        for y in 0..input.height as i32 {
            for x in 0..input.width as i32 {
                simulation.update(x, y, &mut already_flashed);
            }
        }

        for y in 0..input.height as i32 {
            for x in 0..input.width as i32 {
                let i = simulation.to_index(x, y);
                if simulation.grid[i] > 9 {
                    simulation.grid[i] = 0;
                }
            }
        }

        flashes += already_flashed.len() as i32;
    }

    flashes
}

fn part2(input: &Input) -> i32 {
    let mut simulation = Sim::new(input.width, input.height, input.grid.clone());

    let mut step = 0;
    loop {
        step += 1;

        let mut already_flashed = Vec::<usize>::new();

        for y in 0..input.height as i32 {
            for x in 0..input.width as i32 {
                simulation.update(x, y, &mut already_flashed);
            }
        }

        for y in 0..input.height as i32 {
            for x in 0..input.width as i32 {
                let i = simulation.to_index(x, y);
                if simulation.grid[i] > 9 {
                    simulation.grid[i] = 0;
                }
            }
        }

        if already_flashed.len() == input.width * input.height {
            return step;
        }
    }
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input, 100));
    println!("{}", part2(&input));
}
