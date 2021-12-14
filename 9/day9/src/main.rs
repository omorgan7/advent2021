use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Debug)]
struct Input {
    width: i32,
    height: i32,
    heightmaps: Vec<i8>,
}

fn get_input() -> Input {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut bytes = Vec::<u8>::new();
    buf_reader.read_to_end(&mut bytes);

    let height = bytes.iter().filter(|b| **b == b'\n').count() as i32 + 1;
    let width = bytes.len() as i32 / height;

    Input {
        width: width,
        height: height,
        heightmaps: bytes
            .iter()
            .filter_map(|b| {
                if *b != b'\n' {
                    Some((*b - b'0') as i8)
                } else {
                    None
                }
            })
            .collect(),
    }
}

struct ValidIndex {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    called: i32,

    normal: Vec<i32>,
    top_right: Vec<i32>,
    top_left: Vec<i32>,
    bottom_left: Vec<i32>,
    bottom_right: Vec<i32>,
    top_row: Vec<i32>,
    bottom_row: Vec<i32>,
    left_column: Vec<i32>,
    right_column: Vec<i32>,
}

impl ValidIndex {
    fn reset(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.called = 0;
    }
    fn new(width: i32, height: i32) -> ValidIndex {
        let called = 0;
        let normal = vec![1, -1, width, -width];

        let x = 0;
        let y = 0;

        // boundary conditions
        let top_right = vec![-1, width];
        let top_left = vec![1, width];
        let bottom_left = vec![1, -width];
        let bottom_right = vec![-1, -width];
        let top_row = vec![-1, 1, width];
        let bottom_row = vec![-1, 1, -width];
        let left_column = vec![1, -width, width];
        let right_column = vec![-1, -width, width];

        ValidIndex {
            width,
            height,
            x,
            y,
            called,
            normal,
            top_right,
            top_left,
            bottom_left,
            bottom_right,
            top_row,
            bottom_row,
            left_column,
            right_column,
        }
    }

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.called == 4 {
            return None;
        }

        let width = self.width;
        let height = self.height;

        let mut slice = &self.normal;

        let x = self.x;
        let y = self.y;
        if x == 0 {
            if y == 0 {
                slice = &self.top_left;
            } else if y == height - 1 {
                slice = &self.bottom_left;
            } else {
                slice = &self.left_column;
            }
        } else if y == 0 {
            if x == width - 1 {
                slice = &self.top_right;
            } else {
                slice = &self.top_row;
            }
        } else if y == height - 1 {
            if x == width - 1 {
                slice = &self.bottom_right;
            } else {
                slice = &self.bottom_row;
            }
        } else if x == width - 1 {
            slice = &self.right_column;
        } else {
            slice = &self.normal;
        }

        // returns optional.none if out of bounds - what we want.
        let result = {
            if self.called < slice.len() as i32 {
                Some(slice[self.called as usize])
            } else {
                None
            }
        };

        self.called += 1;
        if result.is_none() {
            return None;
        }

        Some((
            x + result.unwrap() % self.width,
            y + result.unwrap() / self.width,
        ))
    }
}

fn part1(input: &Input) -> i32 {
    let mut result = 0i32;
    let mut indexer = ValidIndex::new(input.width, input.height);

    for y in 0..input.height {
        for x in 0..input.width {
            let mut is_minima = true;

            indexer.reset(x, y);

            loop {
                if let Some((new_x, new_y)) = indexer.next() {
                    is_minima &= (input.heightmaps[(x + input.width * y) as usize]
                        < input.heightmaps[(new_x + input.width * new_y) as usize]);
                } else {
                    break;
                }
                if !is_minima {
                    break;
                }
            }

            if is_minima {
                result += 1 + input.heightmaps[(x + input.width * y) as usize] as i32;
            }
        }
    }
    result
}

fn part2(input: &Input) -> i32 {
    let mut heightmaps = input.heightmaps.clone();

    let mut indexer = ValidIndex::new(input.width, input.height);

    let mut reefs = Vec::<i32>::new();

    let width = input.width;
    let height = input.height;

    for y in 0..input.height {
        for x in 0..input.width {
            if heightmaps[(x + width * y) as usize] == 9 {
                continue;
            }

            let mut is_minima = true;

            let mut stack = VecDeque::<(i32, i32)>::new();
            indexer.reset(x, y);

            loop {
                if let Some((new_x, new_y)) = indexer.next() {
                    is_minima &= (heightmaps[(x + width * y) as usize]
                        < heightmaps[(new_x + width * new_y) as usize]);
                } else {
                    break;
                }
                if !is_minima {
                    break;
                }
            }

            if is_minima {
                indexer.reset(x, y);
                let mut reef_size = 1;
                loop {
                    if let Some((new_x, new_y)) = indexer.next() {
                        if heightmaps[(new_x + width * new_y) as usize] != 9 {
                            stack.push_back((new_x, new_y));
                        }
                    } else {
                        break;
                    }
                }

                let mut visited = Vec::<i32>::new();
                visited.push(x + y * width);

                while !stack.is_empty() {
                    reef_size += 1;
                    let (target_x, target_y) = stack.pop_front().unwrap();

                    indexer.reset(target_x, target_y);

                    loop {
                        if let Some((new_x, new_y)) = indexer.next() {
                            if heightmaps[(new_x + width * new_y) as usize] != 9
                                && heightmaps[(new_x + width * new_y) as usize]
                                    > heightmaps[(target_x + width * target_y) as usize]
                                && visited
                                    .iter()
                                    .find(|x| **x == (new_x + width * new_y))
                                    .is_none()
                            {
                                if stack
                                    .iter()
                                    .find(|x| x.0 == new_x && x.1 == new_y)
                                    .is_none()
                                {
                                    stack.push_back((new_x, new_y));
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    visited.push(target_x + target_y * width);
                }
                reefs.push(reef_size);
            }
        }
    }

    let mut max = [0i32; 3];

    for x in &reefs {
        if *x > max[0] {
            max[2] = max[1];
            max[1] = max[0];
            max[0] = *x;
        } else if *x > max[1] {
            max[2] = max[1];
            max[1] = *x;
        } else if *x > max[2] {
            max[2] = *x;
        }
    }

    max.iter().product()
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
