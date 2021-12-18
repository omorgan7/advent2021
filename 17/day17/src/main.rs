#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Input {
    min: Vec2,
    max: Vec2,
}

fn get_input() -> Input {
    //Input { min : Vec2 {x: 20, y: -10}, max: Vec2 {x: 30, y: -5 } }
    Input {
        min: Vec2 { x: 96, y: -144 },
        max: Vec2 { x: 125, y: -98 },
    }
}

fn part1(input: &Input) -> i32 {
    let min = input.min;
    let max = input.max;

    let y = |v0, t| v0 * t;
    let x = |v0, t| v0 * t;

    let mut best = -2147483648;
    let step = 100000;

    let mut count = 0;
    for yy in 0..step {
        for xx in 0..=max.x {
            let mut vx = xx;
            let mut vy = yy - step / 2;

            let mut converged = false;
            let mut sx = 0;
            let mut sy = 0;

            let mut it_best = -2147483648;

            loop {
                let (new_sx, new_sy): (i32, i32) = (sx + x(vx, 1), sy + y(vy, 1));

                vx = if vx > 0 {
                    vx - 1
                } else if vx == 0 {
                    vx
                } else {
                    vx + 1
                };
                vy -= 1;

                if new_sx < min.x {
                    // we're going the wrong way
                    if (new_sx - min.x).abs() > (sx - min.x).abs() {
                        break;
                    }
                }

                // x velocity is 0 and we haven't reached goal yet.
                if new_sx == sx && new_sx < min.x {
                    break;
                }

                sx = new_sx;
                sy = new_sy;

                if sy > it_best {
                    it_best = sy;
                }

                if sx < min.x {
                    continue;
                }

                if sx >= min.x && sx <= max.x && sy >= min.y && sy <= max.y{
                    converged = true;
                    break;
                }

                // have missed the goal.
                if sy < min.y || sx > max.x {
                    break;
                }
            }

            if converged {
                count += 1;
                if it_best > best {
                    best = it_best;
                }
            }
        }
    }

    println!("{}", best);
    println!("{}", count);
    0
}

fn main() {
    part1(&get_input());
}
