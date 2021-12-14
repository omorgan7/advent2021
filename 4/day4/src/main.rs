use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom::Current;
use std::string::String;
use std::vec::Vec;

struct Input {
    callouts: Vec<i32>,
    boards: Vec<[i32; 25]>,
}

fn is_winner(board: [i32; 25]) -> bool {
    // rows
    for y in 0..5 {
        if board[5 * y..].iter().take(5).all(|num| *num == -1) {
            return true;
        }
    }

    for x in 0..5 {
        if board[x..].iter().step_by(5).take(5).all(|num| *num == -1) {
            return true;
        }
    }
    return false;
}

fn get_input() -> Input {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut line = String::new();
    buf_reader.read_line(&mut line);
    let callout: Vec<i32> = line
        .split(",")
        .filter_map(|num| num.to_string().parse::<i32>().ok())
        .collect();

    let mut boards = Vec::<[i32; 25]>::new();

    'high: loop {
        let mut board = [0i32; 25];

        buf_reader.seek(Current(1));

        for y in 0..5 {
            let mut line = String::new();
            buf_reader.read_line(&mut line);

            if line.is_empty() {
                break 'high;
            }

            line = line.replace("\n", "");
            let mut numbers = line.split(" ");
            for x in 0..5 {
                loop {
                    let it = numbers.next();
                    match it.unwrap().to_string().replace(" ", "").parse::<i32>() {
                        Ok(val) => {
                            board[5 * y + x] = val;
                            break;
                        }
                        Err(e) => continue,
                    }
                }
            }
        }
        boards.push(board);
    }
    Input {
        boards: boards,
        callouts: callout,
    }
}

fn part1(input: &Input) -> i32 {
    let mut boards_copy = input.boards.clone();

    let winning_board: ([i32; 25], i32) = {
        let mut winner = ([0i32; 25], 0i32);
        'outer: for call in &input.callouts {
            for board in &mut boards_copy.iter_mut() {
                let mut search = board.iter_mut().find(|num| **num == *call);
                if search.is_some() {
                    let mut value = search.unwrap();
                    *value = -1;
                }
                if is_winner(*board) {
                    winner = (*board, *call);
                    break 'outer;
                }
            }
        }
        winner
    };

    winning_board.0.iter().fold(
        0,
        |total, boards| {
            if *boards == -1 {
                total
            } else {
                total + boards
            }
        },
    ) * winning_board.1
}

fn part2(input: &Input) -> i32 {
    let mut boards_copy = input.boards.clone();

    let winning_board: ([i32; 25], i32) = {
        let mut winners: Vec<usize> = (0..input.boards.len()).map(|a| a).collect();
        let mut winner = ([0i32; 25], 0i32);
        'outer1: for call in &input.callouts {
            for board in &mut boards_copy.iter_mut().enumerate() {
                let mut search = board.1.iter_mut().find(|num| **num == *call);
                if search.is_some() {
                    let mut value = search.unwrap();
                    *value = -1;
                }
                if is_winner(*board.1) {
                    winners.retain(|x| *x != board.0);
                    if winners.len() == 1 {
                        break 'outer1;
                    }
                }
            }
        }

        'outer2: for call in &input.callouts {
            let mut search = boards_copy[winners[0]]
                .iter_mut()
                .find(|num| **num == *call);
            if search.is_some() {
                let mut value = search.unwrap();
                *value = -1;
            }
            if is_winner(boards_copy[winners[0]]) {
                winner = (boards_copy[winners[0]], *call);
                break 'outer2;
            }
        }
        winner
    };

    winning_board.0.iter().fold(
        0,
        |total, boards| {
            if *boards == -1 {
                total
            } else {
                total + boards
            }
        },
    ) * winning_board.1
}

fn main() {
    let input = get_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
