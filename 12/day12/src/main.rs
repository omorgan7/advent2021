use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Debug)]
struct Node {
    name: String,
    is_big: bool,
    connections: Vec<usize>,
}

fn is_uppercase(s: &String) -> bool {
    *s == s.to_uppercase()
}

fn get_input() -> Vec<Node> {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut nodes = Vec::<Node>::new();

    for line in buf_reader.lines() {
        let new_line = line.unwrap();
        let mut split = new_line.split("-");
        let first = split.next().unwrap().to_string();
        let second = split.next().unwrap().to_string();

        if nodes.iter().find(|node| *node.name == first).is_none() {
            nodes.push(Node {
                name: first.clone(),
                is_big: is_uppercase(&first),
                connections: Vec::<usize>::new(),
            });
            let first_index = nodes.len() - 1;

            if let Some((i, _)) = nodes
                .iter()
                .enumerate()
                .find(|(_, node)| *node.name == second)
            {
                nodes[first_index].connections.push(i);
                nodes[i].connections.push(first_index);
            } else {
                nodes.push(Node {
                    name: second.clone(),
                    is_big: is_uppercase(&second),
                    connections: vec![first_index],
                });
                let second_index = nodes.len() - 1;
                nodes[first_index].connections.push(second_index);
            }
        } else if nodes.iter().find(|node| *node.name == second).is_none() {
            nodes.push(Node {
                name: second.clone(),
                is_big: is_uppercase(&second),
                connections: Vec::<usize>::new(),
            });
            let second_index = nodes.len() - 1;

            if let Some((i, _)) = nodes
                .iter()
                .enumerate()
                .find(|(_, node)| *node.name == first)
            {
                nodes[second_index].connections.push(i);
                nodes[i].connections.push(second_index);
            } else {
                nodes.push(Node {
                    name: first.clone(),
                    is_big: is_uppercase(&first),
                    connections: vec![second_index],
                });
                let first_index = nodes.len() - 1;
                nodes[second_index].connections.push(first_index);
            }
        } else {
            let first_index = nodes
                .iter()
                .enumerate()
                .find(|(_, node)| *node.name == first)
                .unwrap()
                .0;
            let second_index = nodes
                .iter()
                .enumerate()
                .find(|(_, node)| *node.name == second)
                .unwrap()
                .0;

            nodes[first_index].connections.push(second_index);
            nodes[second_index].connections.push(first_index);
        }
    }

    nodes
}

fn visit_p1(
    node: usize,
    end: usize,
    input: &Vec<Node>,
    visited: &mut Vec<usize>,
    end_count: &mut i32,
) {
    if visited.iter().find(|i| **i == node).is_some() {
        return;
    }

    if node == end {
        *end_count += 1;
        return;
    }

    if !input[node].is_big {
        visited.push(node);
    }

    for child in &input[node].connections {
        let mut visit_clone = visited.clone();
        visit_p1(*child, end, input, &mut visit_clone, end_count);
    }
}

fn visit_p2(
    start: usize,
    node: usize,
    end: usize,
    input: &Vec<Node>,
    visited: &mut HashMap<usize, usize>,
    end_count: &mut i32,
) {
    if visited[&node] == 2 {
        return;
    }

    if visited[&node] == 1 && (visited.values().any(|v| *v == 2) || node == start) {
        return;
    }

    if node == end {
        *end_count += 1;
        return;
    }

    if !input[node].is_big {
        if node == start {
            *visited.get_mut(&node).unwrap() = 1;
        } else {
            *visited.get_mut(&node).unwrap() = visited[&node] + 1;
        }
    }

    for child in &input[node].connections {
        let mut visit_clone = visited.clone();
        visit_p2(start, *child, end, input, &mut visit_clone, end_count);
    }
}

fn part1(input: &Vec<Node>) -> i32 {
    let mut visited = Vec::<usize>::new();
    let mut count = 0i32;

    let start = input
        .iter()
        .enumerate()
        .find(|(_, node)| node.name == "start")
        .unwrap()
        .0;
    let end = input
        .iter()
        .enumerate()
        .find(|(_, node)| node.name == "end")
        .unwrap()
        .0;
    visit_p1(start, end, input, &mut visited, &mut count);

    count
}

fn part2(input: &Vec<Node>) -> i32 {
    let mut visited = HashMap::<usize, usize>::new();

    for (i, _) in input.iter().enumerate() {
        visited.insert(i, 0);
    }

    let mut count = 0i32;

    let start = input
        .iter()
        .enumerate()
        .find(|(_, node)| node.name == "start")
        .unwrap()
        .0;
    let end = input
        .iter()
        .enumerate()
        .find(|(_, node)| node.name == "end")
        .unwrap()
        .0;
    visit_p2(start, start, end, input, &mut visited, &mut count);

    count
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
