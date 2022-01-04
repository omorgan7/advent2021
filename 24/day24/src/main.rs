use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
enum Register {
    X,
    Y,
    Z,
    W,
}

#[derive(Debug, Clone, Copy)]
struct UnaryOp {
    register: Register,
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Reg(Register),
    Value(i64),
}

#[derive(Debug, Clone, Copy)]
struct BinaryOp {
    a0: Operand,
    a1: Operand,
    op: fn(i64, i64) -> i64,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Unary(UnaryOp),
    Binary(BinaryOp),
}

type Input = Vec<Instruction>;

fn get_input() -> Input {
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let lines = buf_reader
        .lines()
        .filter_map(|x| Some(x.ok()?.chars().collect::<Vec<char>>()))
        .collect::<Vec<Vec<char>>>();

    let to_reg = |s: char| -> Option<Register> {
        match s {
            'x' => Some(Register::X),
            'y' => Some(Register::Y),
            'z' => Some(Register::Z),
            'w' => Some(Register::W),
            _ => None,
        }
    };

    lines
        .iter()
        .map(|line| -> Instruction {
            let first = line[0..3].iter().collect::<String>();

            match first.as_str() {
                "inp" => Instruction::Unary(UnaryOp {
                    register: to_reg(line[4]).unwrap(),
                }),
                _ => {
                    let a0: Operand = match to_reg(line[4]) {
                        Some(t) => Operand::Reg(t),
                        None => panic!(),
                    };
                    let a1: Operand = match to_reg(line[6]) {
                        Some(t) => Operand::Reg(t),
                        None => Operand::Value(line[6..].iter().collect::<String>().parse::<i64>().unwrap())
                    };
                    return match first.as_str() {
                        "add" => Instruction::Binary(BinaryOp {
                            a0,
                            a1,
                            op: |a, b| a + b,
                        }),
                        "mul" => Instruction::Binary(BinaryOp {
                            a0,
                            a1,
                            op: |a, b| a * b,
                        }),
                        "div" => Instruction::Binary(BinaryOp {
                            a0,
                            a1,
                            op: |a, b| a / b,
                        }),
                        "mod" => Instruction::Binary(BinaryOp {
                            a0,
                            a1,
                            op: |a, b| a % b,
                        }),
                        "eql" => Instruction::Binary(BinaryOp {
                            a0,
                            a1,
                            op: |a, b| (a == b) as i64,
                        }),
                        _ => panic!(),
                    };
                }
            }
        })
        .collect()
}

fn to_variable<'a>(
    r: Register,
    x: &'a mut i64,
    y: &'a mut i64,
    z: &'a mut i64,
    w: &'a mut i64,
) -> &'a mut i64 {
    match r {
        Register::X => x,
        Register::Y => y,
        Register::Z => z,
        Register::W => w,
    }
}

struct Search {
    value : i64
}

impl Search {
    fn new() -> Search {
        Search { value: 99999999999999 }
    }

    fn digit_by_10(x: i64) -> Option<i64> {

        let mut divisor = 10000000000000;
        loop {
            if (x / divisor) % 10 == 0 {
                return Some(divisor);
            }

            divisor /= 10;

            if divisor == 0 {
                break;
            }
        }

        None
    }
}

impl Iterator for Search {
    type Item = i64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let old = self.value;
        let mut new = self.value - 1;

        // while let Some(power) = Search::digit_by_10(new) {
        //     println!("power: {}", power);
        //     new -= power;
        // }
        self.value = new;

        if new < 99999999999998 {
            return None;
        }

        Some(old)
    }
}

fn part1(input: &Input) -> i64 {

    let mut result = 0i64;
    let to_program_input = |x| -> [i64; 14] {
        let mut tmp = [0i64; 14];
        let mut divisor = 1;
        for i in 0..14 {
            tmp[i] = (x / divisor) % 10;
            divisor *= 10;
        }

        tmp
    };

    for n in Search::new() {

        let program_input = to_program_input(n);
        println!("N: {}", n);

        if program_input.iter().any(|x| *x == 0) {
            continue;
        }

        let mut program_counter = 0;

        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        let mut w = 0;

        for p in input {
            match p {
                Instruction::Unary(op) => {
                    *to_variable(op.register, &mut x, &mut y, &mut z, &mut w) =
                        program_input[program_counter];
                    println!("{} {} {} {}", x, y, z, w);
                    program_counter += 1;
                }
                Instruction::Binary(op) => {
                    let a0_reg = if let Operand::Reg(tmp) = op.a0 {
                        tmp
                    } else {
                        panic!();
                    };

                    let a0: i64 = *to_variable(a0_reg, &mut x, &mut y, &mut z, &mut w);
                    let a1 = match op.a1 {
                        Operand::Reg(tmp) => *to_variable(tmp, &mut x, &mut y, &mut z, &mut w),
                        Operand::Value(v) => v,
                    };

                    *to_variable(a0_reg, &mut x, &mut y, &mut z, &mut w) = (op.op)(a0, a1);
                    println!("{} {} {} {}", x, y, z, w);
                }
            }
        }

        if z == 0 {
            result = n;
            break;
        }
    }
    
    result
}

fn part2(input: &Input) -> i64 {
    0
}

fn main() {
    let input = get_input();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
