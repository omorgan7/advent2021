use std::cmp::Ordering;
use std::collections::BinaryHeap;
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
    let buf_reader = BufReader::new(File::open("input.txt").unwrap());

    let mut height = 0;
    let grid = buf_reader
        .lines()
        .filter_map(|x| {
            height += 1;
            Some(
                x.unwrap()
                    .chars()
                    .filter_map(|c| Some(c.to_digit(10)? as u8))
                    .collect::<Vec<u8>>(),
            )
        })
        .flatten()
        .collect::<Vec<u8>>();

    let width = grid.len() / height;

    Input {
        width: width,
        height: height,
        grid: grid,
    }
}

struct State {
    cost: i32,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        other.node == self.node
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Vec<u8>, width: usize, height: usize) -> i32 {
    let in_bounds = |current: usize, next: (i32, i32)| -> bool {
        let x = ((current as usize) % width) as i32;
        let y = ((current as usize) / width) as i32;

        (x + next.0) >= 0
            && (x + next.0) < width as i32
            && (y + next.1) >= 0
            && (y + next.1) < height as i32
    };

    let mut heap = BinaryHeap::<State>::new();
    let mut distances = Vec::<i32>::new();
    let mut nodes = Vec::<i32>::new();

    for y in 0..height {
        for x in 0..width {
            nodes.push((x + width * y) as i32);
            distances.push(i32::max_value());
        }
    }

    heap.push(State { cost: 0, node: 0 });
    distances[0] = 0;
    while let Some(s) = heap.pop() {
        let min_node_index = s.node;
        let dist = s.cost;

        if min_node_index == width * height - 1 {
            break;
        }

        if dist > distances[width * height - 1] {
            continue;
        }

        for child in [(-1i32, 0), (0, -1i32), (1i32, 0), (0, 1i32)].iter() {
            if !in_bounds(min_node_index, *child) {
                continue;
            }

            let next_index = (min_node_index as i32 + child.0 + width as i32 * child.1) as usize;

            let alternative = dist + (grid[next_index] as i32);
            let current = distances[next_index];

            if alternative < current {
                distances[next_index] = alternative;
                heap.push(State {
                    cost: alternative,
                    node: next_index,
                });
            }
        }
    }

    distances[width * height - 1]
}

fn part1(input: &Input) -> i32 {
    dijkstra(&input.grid, input.width, input.height)
}

fn part2(input: &Input) -> i32 {
    let new_width = input.width * 5;
    let new_height = input.height * 5;

    let mut grids = Vec::<Vec<u8>>::new();

    for yy in 0..5 {
        for xx in 0..5 {
            if xx == 0 && yy == 0 {
                grids.push(input.grid.clone());
                continue;
            }
            grids.push(Vec::<u8>::new());

            let grid_off = {
                if xx == 0 {
                    5 * (yy - 1)
                } else {
                    5 * yy + xx - 1
                }
            };
            let back = grids.len() - 1;
            let (g0, g1) = grids.split_at_mut(back);
            let old_grid = &g0[grid_off];
            let new_grid = &mut g1[0];

            for y in 0..input.height {
                for x in 0..input.width {

                    let old = old_grid[x + input.width * y];

                    if old + 1 > 9 {
                        new_grid.push(1);
                    } else {
                        new_grid.push(old + 1);
                    }
                }
            }
        }
    }

    let mut new_grid = Vec::<u8>::new();

    for yy in 0..5 {
        for y in 0..input.height {
            for xx in 0..5 {
                for x in 0..input.width {
                    new_grid.push(grids[xx + 5 * yy][x + input.width * y]);
                }
            }
        }
    }
    dijkstra(&new_grid, new_width, new_height)
}

fn main() {
    let input = get_input();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
