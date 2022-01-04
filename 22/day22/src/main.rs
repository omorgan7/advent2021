use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;
use std::hash::BuildHasherDefault;

extern crate regex;
use regex::{Captures, Regex};

extern crate hashers;
use hashers::fx_hash::FxHasher;

#[derive(Debug, Clone, Copy)]
struct Step {
    on: u8,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

type Input = Vec<Step>;

fn get_input() -> Input {
    let buf_reader = BufReader::new(File::open("input.txt").unwrap());

    let re = Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();

    buf_reader
        .lines()
        .filter_map(|line| {
            let l = line.ok()?;
            let tmp = re.captures_iter(&l).collect::<Vec<Captures>>();
            let caps = &tmp[0];
            let on = (caps.get(1)?.as_str() == "on") as u8;
            let x = (
                caps.get(2)?.as_str().parse::<i32>().ok()?,
                caps.get(3)?.as_str().parse::<i32>().ok()?,
            );
            let y = (
                caps.get(4)?.as_str().parse::<i32>().ok()?,
                caps.get(5)?.as_str().parse::<i32>().ok()?,
            );
            let z = (
                caps.get(6)?.as_str().parse::<i32>().ok()?,
                caps.get(7)?.as_str().parse::<i32>().ok()?,
            );

            Some(Step { on, x, y, z })
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Box {
    min: [i32; 3],
    max: [i32; 3]
}

impl Box {
    fn from_tuple(x: (i32, i32), y: (i32, i32), z: (i32, i32)) -> Box {
        let x0 = if x.1 > x.0 { x.0 } else { x.1 };
        let x1 = if x.1 > x.0 { x.1 } else { x.0 };

        let y0 = if y.1 > y.0 { y.0 } else { y.1 };
        let y1 = if y.1 > y.0 { y.1 } else { y.0 };

        let z0 = if z.1 > z.0 { z.0 } else { z.1 };
        let z1 = if z.1 > z.0 { z.1 } else { z.0 };

        Box { min: [x0, y0, z0], max: [x1, y1, z1] }
    }

    fn new(x: [i32; 2], y: [i32; 2], z: [i32; 2]) -> Box {
        let x0 = if x[1] > x[0] { x[0] } else { x[1] };
        let x1 = if x[1] > x[0] { x[1] } else { x[0] };

        let y0 = if y[1] > y[0] { y[0] } else { y[1] };
        let y1 = if y[1] > y[0] { y[1] } else { y[0] };

        let z0 = if z[1] > z[0] { z[0] } else { z[1] };
        let z1 = if z[1] > z[0] { z[1] } else { z[0] };

        Box { min: [x0, y0, z0], max: [x1, y1, z1] }
    }

    fn intersection(a: &Box, b: &Box) -> Option<Box> {

        let in_bounds = |s, t, axis| {
            a.max[axis] > b.min[axis] && a.max[axis] < b.max[axis]
        };

        if !(in_bounds(a, b, 0) && in_bounds(a, b, 1) && in_bounds(a, b, 2)) {
            return None;
        }
        
        let x0 = std::cmp::max(a.min[0], b.min[0]);
        let x1 = std::cmp::min(a.max[0], b.max[0]);

        let y0 = std::cmp::max(a.min[1], b.min[1]);
        let y1 = std::cmp::min(a.max[1], b.max[1]);

        let z0 = std::cmp::max(a.min[2], b.min[2]);
        let z1 = std::cmp::min(a.max[2], b.max[2]);

        Some(Box { min: [x0, y0, z0], max: [x1, y1, z1] })
    }

    fn subdivisions(a: &Box, b: &Box) -> [Box; 8] {
        [
            Box::new(x: [a.min.x, b.min.x], y: [a.min.y, b.min.y], z: [a.min.z, b.min.z]),
            Box::new(x: [a.min.x, b.min.x], y: [b.min.y, a.max.y], z: [a.min.z, b.min.z]),
            Box::new(x: [b.min.x, a.max.x], y: [b.min.y, a.max.y], z: [a.min.z, b.min.z]),
            Box::new(x: [b.min.x, a.max.x], y: [a.min.y, b.min.y], z: [a.min.z, b.min.z]),

            Box::new(x: [a.min.x, b.min.x], y: [a.min.y, b.min.y], z: [b.min.z, a.min.z]),
            Box::new(x: [a.min.x, b.min.x], y: [b.min.y, a.max.y], z: [b.min.z, a.min.z]),
            Box::new(x: [b.min.x, a.max.x], y: [b.min.y, a.max.y], z: [b.min.z, a.min.z]),
            Box::new(x: [b.min.x, a.max.x], y: [a.min.y, b.min.y], z: [b.min.z, ax  .min.z]),
        ]
    }

    fn volume(&self) -> i64 {
        (self.max[0] as i64 - self.min[0] as i64) *
        (self.max[1] as i64 - self.min[1] as i64) *
        (self.max[2] as i64 - self.min[2] as i64)
    }
}

fn part1(input: &Input) -> i64 {
    let mut cubes = Vec::<Box>::new();

    cubes.push(Box::from_tuple(input[0].x, input[0].y, input[0].z));

    
    for step in input[1..].iter() {
        let mut new_cubes = Vec::<Box>::new();

        let boxx = Box::from_tuple(step.x, step.y, step.z);
        for c in cubes {
            if Box::intersection(&boxx, &c).is_none() {

            }
            else {
                new_cubes.push(c);
            }
        }
    }

    cubes[1..].iter().fold(cubes[0], |prev, curr| {
        println!("{:?}", );
        prev
    });

    0
}

fn part2(input: &Input) -> i64 {
    0
}

fn main() {
    let input = get_input();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
