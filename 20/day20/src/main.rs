use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<u8>,
    fill: u8,
}

impl Grid {
    fn pad_edges(&self, expansion: usize) -> Grid {
        let new_width = self.width + 2 * expansion;
        let new_height = self.height + 2 * expansion;

        let mut new_grid = Vec::<u8>::new();

        // top rows
        for _ in 0..new_width {
            for _ in 0..expansion {
                new_grid.push(self.fill);
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if x == 0 {
                    for _ in 0..expansion {
                        new_grid.push(self.fill);
                    }
                    new_grid.push(self.grid[x + y * self.width]);
                } else if x == self.width - 1 {
                    new_grid.push(self.grid[x + y * self.width]);
                    for _ in 0..expansion {
                        new_grid.push(self.fill);
                    }
                } else {
                    new_grid.push(self.grid[x + y * self.width]);
                }
            }
        }

        for _ in 0..new_width {
            for _ in 0..expansion {
                new_grid.push(self.fill);
            }
        }

        Grid {
            width: new_width,
            height: new_height,
            grid: new_grid,
            fill: self.fill,
        }
    }

    fn point_internal(&self, x: i32, y: i32) -> u8 {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.width as i32 {
            self.fill
        } else {
            self.grid[x as usize + y as usize * self.width]
        }
    }

    fn points_at(&self, x: usize, y: usize) -> [u8; 9] {
        let mut output = [0u8; 9];

        let xx = x as i32;
        let yy = y as i32;

        output[0] = self.point_internal(xx - 1, yy - 1);
        output[1] = self.point_internal(xx, yy - 1);
        output[2] = self.point_internal(xx + 1, yy - 1);

        output[3] = self.point_internal(xx - 1, yy);
        output[4] = self.point_internal(xx, yy);
        output[5] = self.point_internal(xx + 1, yy);

        output[6] = self.point_internal(xx - 1, yy + 1);
        output[7] = self.point_internal(xx, yy + 1);
        output[8] = self.point_internal(xx + 1, yy + 1);

        output
    }

    fn replace(&mut self, x: usize, y: usize, v: u8) {
        self.grid[x + y * self.width] = v;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let xx = self.grid[x + self.width * y];
                print!("{}", if xx == 0 { '.' } else { '#' });
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug)]
struct Input {
    grid: Grid,
    decoder: Vec<u8>,
}

fn get_input() -> Input {
    let buf_reader = BufReader::new(File::open("input.txt").unwrap());
    let lines = buf_reader
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>();

    let decoder = lines[0].chars().map(|c| (c == '#') as u8).collect();

    let height = lines[2..].len();

    let grid = lines[2..]
        .iter()
        .map(|line| line.chars().map(|c| (c == '#') as u8).collect::<Vec<u8>>())
        .flatten()
        .collect::<Vec<u8>>();

    let width = grid.len() / height;

    Input {
        grid: Grid {
            width,
            height,
            grid,
            fill: 0,
        },
        decoder,
    }
}

fn part1(input: &Input, iterations: i32) -> i64 {
    let mut new_grid = input.grid.clone();

    let to_num = |x: [u8; 9]| {
        x.iter()
            .rev()
            .fold((0i32, 1i32), |(sum, mult), b| {
                (sum + mult * (*b as i32), mult * 2)
            })
            .0
    };

    new_grid = new_grid.pad_edges(1);
    let mut grid = new_grid.clone();

    for _ in 0..iterations {
        for y in 0..new_grid.height {
            for x in 0..new_grid.width {
                let points = new_grid.points_at(x, y);
                grid.replace(x, y, input.decoder[to_num(points) as usize]);
            }
        }

        // delete this for the example input
        grid.fill = if grid.fill == 0 { 1 } else { 0 };

        new_grid = grid.pad_edges(2);
        grid = new_grid.clone();
    }

    new_grid.print();
    new_grid.grid.iter().fold(0i64, |t, p| t + *p as i64)
}

fn main() {
    let input = get_input();

    println!("{}", part1(&input, 2));
    println!("{}", part1(&input, 50));
}
