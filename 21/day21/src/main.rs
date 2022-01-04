use std::vec::Vec;

#[derive(Debug, Copy, Clone)]
struct Player {
    score : i32,
    position: i32,
    win_count : i32
}

type Input = (Player, Player);


struct QuantumDice {
    number : i32
}

impl QuantumDice {
    fn roll(&mut self) -> i32 {
        let old = self.number;
        self.number += 1;

        if self.number > 3 {
            self.number = 1;
        }

        old
    }

    fn new() -> QuantumDice {
        QuantumDice { number: 1 }
    }
}

struct Dice {
    number: i32,
    roll_count: i32
}

impl Dice {
    fn roll(&mut self) -> i32 {
        let old = self.number;
        self.number += 1;
        self.roll_count += 1;

        old
    }
    fn new() -> Dice {
        Dice { number: 1, roll_count : 0 }
    }
}

fn get_input() -> Input {
    (Player { score: 0, position: 4, win_count: 0 }, Player { score: 0, position: 8, win_count: 0 })
    // (Player { score: 0, position: 9, win_count: 0 }, Player { score: 0, position: 4, win_count: 0 })
}

fn part1(input: &Input) -> i64 {
    
    let mut players : (Player, Player) = (input.0, input.1);

    let mut losing_score = 0;

    let mut dice = Dice::new();

    let update_score = |player: &mut Player, dice : &mut Dice| {
        let rolls = [dice.roll(), dice.roll(), dice.roll()];

        for roll in rolls { 

            let roll10 = roll / 10;
            let roll10r = roll % 10;

            let mut new_position = player.position;
            for _ in 0..roll10 {
                new_position += 10;
                if new_position > 10 {
                    new_position -= 10;
                }
            }

            new_position += roll10r;
            if new_position > 10 {  
                new_position -= 10;
            }
            player.position = new_position;

        }
        player.score += player.position;
    };

    loop {
        update_score(&mut players.0, &mut dice);
        if players.0.score >= 1000 {
            losing_score = players.1.score;
             break;
        }

        update_score(&mut players.1, &mut dice);
        if players.1.score >= 1000 {
            losing_score = players.0.score;
             break;
        }
    }


    losing_score as i64 * dice.roll_count as i64
}

fn update_score_pt2(p0: &mut Player, p1: &mut Player, dice_roll: i32, mut p0_turn: i32, mut p1_turn: i32, turn: 0) -> (Player, Player)
{
    if p0.score >= 21 {
        p0.win_count += 1;
        return (*p0, *p1);
    }

    if p1.score >= 21 {
        p1.win_count += 1;
        return (*p0, *p1);
    }

    if p0_turn < 3 {
        p0_turn += 1;

        for roll in 1..=3 {
            update_score_pt2(p0, p1, roll, p0_turn, p1_turn, 0)
        }
    }

    (*,0, * 1)
}

fn part2(input: &Input) -> i64 {
    0
}

fn main() {
    let input = get_input();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
