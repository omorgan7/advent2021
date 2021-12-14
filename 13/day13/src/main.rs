use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::vec::Vec;

#[derive(Debug)]
enum Fold {
    x(usize),
    y(usize)
}

#[derive(Debug)]
struct Input {
    width : usize,
    height : usize,
    grid : Vec::<u8>,
    folds : Vec::<Fold>
}

fn get_input() -> Input {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let lines = buf_reader.lines().filter_map(|x| x.ok()).collect::<Vec::<String>>();

    let dots : Vec::<(usize, usize)>= lines
        .iter()
        .map(|line| -> Vec::<usize> {
            line
                .split(',')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect()
    }).collect::<Vec::<Vec::<usize>>>()
        .iter()
        .filter_map(|x| if x.is_empty() { None } else { Some((x[0], x[1])) })
        .collect();

    let (width, height) = dots.iter().fold((0, 0), |(xm, ym), (x, y)| {
        (std::cmp::max(xm, *x), std::cmp::max(ym, *y))
    });

    let mut grid = Vec::<u8>::new();
    grid.resize((width + 1) * (height + 1), 0);

    for (x, y) in &dots {
        grid[x + (width + 1) * y] = 1;
    }

    let x_folds : Vec::<(usize, usize)> = lines.iter().enumerate().filter_map(|(i, line)| {
        let num = line.splitn(2, "fold along x=").nth(1)?.parse::<usize>();
        Some((i, num.ok()?))
    }).collect();

    let y_folds : Vec::<(usize, usize)> = lines.iter().enumerate().filter_map(|(i, line)| {
        let num = line.splitn(2, "fold along y=").nth(1)?.parse::<usize>();
        Some((i, num.ok()?))
    }).collect();

    let mut folds = Vec::<Fold>::new();
    let mut y = 0usize;
    let mut x = 0usize;
    loop {
        if x < x_folds.len() && y < y_folds.len() {
            if x_folds[x].0 < y_folds[y].0 {
                folds.push(Fold::x(x_folds[x].1));
                x += 1;
            }
            else {
                folds.push(Fold::y(y_folds[y].1));
                y += 1;
            }
        }
        else if y < y_folds.len() {
            folds.push(Fold::y(y_folds[y].1));
            y += 1;
        }
        else if x < x_folds.len() {
            folds.push(Fold::x(x_folds[x].1));
            x += 1;
        }
        else {
            break;
        }
    }

    Input {width: width + 1, height: height + 1, grid : grid, folds: folds}
}

fn part1(input : &Input, target : usize) -> i32 {

    let mut grid = input.grid.clone();
    let mut width = input.width;
    let mut height = input.height;

    for fold in &input.folds[..target] {
        match fold {
            Fold::x(fold_x) => {
                for y in 0..height {
                    for x in *fold_x..width {
                        if grid[x + input.width * y] == 1 {
                            grid[(width / 2 - (x - fold_x)) + input.width * y] = 1;
                        }
                    }
                }
                width /= 2;
            },
            Fold::y(fold_y) => {
                for y in *fold_y..height {
                    for x in 0..width {
                        if grid[x + input.width * y] == 1 {
                            grid[x + input.width * (height / 2 - (y - fold_y))] = 1;
                        }
                    }
                }
                height /= 2;
            }
        }
    }
    
    let mut dot_count = 0;
    for y in 0..height {
        for x in 0..width {
            dot_count += grid[x + input.width * y] as i32;
        }
    }
    for y in 0..height {
        for x in 0..width {
            print!("{}", if grid[x + y * input.width] == 0 { '.' } else { '#' });
        }
        print!("\n");
    }
    print!("-------\n");

    dot_count
}

fn main() {
    let input = get_input();

    println!("{}", part1(&input, 1));
    println!("{}", part1(&input, input.folds.len()));
}
