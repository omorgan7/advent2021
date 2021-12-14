use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

type Single = char;
type Pair = Vec<char>;

#[derive(Debug)]
struct Input {
    protein: HashMap<Pair, i64>,
    mappings: HashMap<Pair, Single>,
}

fn get_input() -> Input {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let lines = buf_reader
        .lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<String>>();

    let mut protein = HashMap::<Pair, i64>::new();
    lines
        .iter()
        .take(1)
        .next()
        .unwrap()
        .as_bytes()
        .windows(2)
        .for_each(|c| {
            (*protein
                .entry(c.iter().map(|cc| *cc as char).collect())
                .or_insert(0) += 1)
        });

    let mappings = lines
        .iter()
        .skip(2)
        .filter_map(|l| {
            let mut split = l.split(" -> ");

            let pair = split.next()?.to_string();
            let single = split.next()?.to_string();

            Some((pair.chars().collect(), single.as_bytes()[0] as Single))
        })
        .collect();

    Input { protein, mappings }
}

fn part1(input: &Input, target: usize) -> i64 {
    let mut old = input.protein.clone();

    for _ in 0..target {
        let mut new = HashMap::<Pair, i64>::new();

        for pair in old.keys() {
            let new_map = input.mappings.get(pair).unwrap();
            let v = old.get(pair).unwrap();

            *new.entry(vec![pair[0], *new_map]).or_insert(0) += v;
            *new.entry(vec![*new_map, pair[1]]).or_insert(0) += v;
        }

        old = new;
    }

    let mut counts = HashMap::<Single, f64>::new();

    for (k, v) in old.iter() {
        for letter in k.iter() {
            *counts.entry(*letter).or_insert(0.0) += (*v as f64) * 0.5;
        }
    }

    let min = counts
        .values()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        .ceil() as i64;
    let max = counts
        .values()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        .ceil() as i64;

    max - min
}

fn main() {
    let input = get_input();

    println!("{}", part1(&input, 10));
    println!("{}", part1(&input, 40));
}
