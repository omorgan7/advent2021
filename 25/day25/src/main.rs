use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;
use std::{thread, time};

extern crate termion;
use termion::clear::All;

const EAST: i8 = 1;
const SOUTH: i8 = 2;

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<i8>
}

type Input = Grid;

impl Grid {
    fn at(&self, x: usize, y: usize) -> i8 {
        if x < self.width && y < self.height {
            return self.grid[x + y * self.width];
        }
        if y == self.height && x != self.width {
            return self.grid[x];
        }
        if x == self.width && y != self.height {
            return self.grid[y * self.width];
        }
        return self.grid[0];
    }

    fn replace(&mut self, x: usize, y: usize, v: i8) {
        let xx : &mut i8 = 
            if x < self.width && y < self.height {
                &mut self.grid[x + y * self.width]
            }
            else if y == self.height && x != self.width {
                &mut self.grid[x]
            }
            else if x == self.width && y != self.height {
                &mut self.grid[y * self.width]
            }
            else {
                &mut self.grid[0]
            };

        *xx = v;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let xx = self.grid[x + self.width * y];
                print!("{}", if xx == EAST { '>' } else if xx == SOUTH { 'v' } else { '.' });
            }
            println!();
        }
        println!();
    }
}

fn get_input() -> Input {
    let buf_reader = BufReader::new(File::open("input.txt").unwrap());
    let lines = buf_reader
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>();


    let height = lines.len();

    let grid = lines
        .iter()
        .map(|line| line.chars().map(|c| if c == 'v' { SOUTH } else if c == '>' { EAST } else { 0 } ).collect::<Vec<i8>>())
        .flatten()
        .collect::<Vec<i8>>();

    let width = grid.len() / height;

    Grid {
        width,
        height,
        grid,
    }
}

fn part1(input: &Input) -> i64 {
    let mut prev = input.clone();

    let mut counter = 0;
    loop {
        let mut new_east = prev.clone();

        for y in 0..input.height {
            for x in 0..input.width {
                if prev.at(x, y) == EAST {
                    if prev.at(x + 1, y) == 0 {
                        new_east.replace(x, y, 0);
                        new_east.replace(x + 1, y, EAST);
                    }
                }
            }
        }

        let mut new = new_east.clone();
        for y in 0..input.height {
            for x in 0..input.width {
                if new_east.at(x, y) == SOUTH {
                    if new_east.at(x, y + 1) == 0 {
                        new.replace(x, y, 0);
                        new.replace(x, y + 1, SOUTH);
                    }
                }
            }
        }

        
        new.print();
        thread::sleep(time::Duration::from_millis(66));


        counter += 1;
        if prev.grid == new.grid {
            break;
        }
        print!("{}", termion::clear::All);
        prev = new.clone();


    }
    counter
}

fn part2(input: &Input) -> i64 {
    0
}

fn main() {
    let input = get_input();

    input.print();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
