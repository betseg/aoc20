#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    Left,
    Right,
    Forward,
}

#[derive(Copy, Clone)]
pub struct Instruction {
    dir: Direction,
    units: isize,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    let mut res = Vec::new();

    for line in input.lines() {
        let dir = match line.chars().next().unwrap() {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            _ => unreachable!(),
        };
        let units = line[1..].parse().unwrap();
        res.push(Instruction { dir, units });
    }

    res
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Instruction]) -> isize {
    struct State {
        dir: Direction,
        x: isize,
        y: isize,
    }

    let mut state = State {
        dir: Direction::East,
        x: 0,
        y: 0,
    };

    for instr in input {
        match instr.dir {
            dir @ Direction::North
            | dir @ Direction::South
            | dir @ Direction::East
            | dir @ Direction::West => {
                // (state.x, state.y) = move_(dir, state.x, state.y, instr.units);
                // RFC #2909
                let (x, y) = move_with_dir(dir, state.x, state.y, instr.units);
                state.x = x;
                state.y = y;
            }
            Direction::Forward => {
                let (x, y) = move_with_dir(state.dir, state.x, state.y, instr.units);
                state.x = x;
                state.y = y;
            }
            dir @ Direction::Left | dir @ Direction::Right => {
                state.dir = rotate(dir, state.dir, instr.units);
            }
        }
    }

    state.x.abs() + state.y.abs()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Instruction]) -> isize {
    let mut waypoint = (10, 1);
    let mut ship_loc = (0, 0);

    for instr in input {
        match instr.dir {
            Direction::Forward => {
                for _ in 0..instr.units {
                    ship_loc.0 += waypoint.0;
                    ship_loc.1 += waypoint.1;
                }
            }
            Direction::North => waypoint.1 += instr.units,
            Direction::South => waypoint.1 -= instr.units,
            Direction::East => waypoint.0 += instr.units,
            Direction::West => waypoint.0 -= instr.units,
            dir @ Direction::Left | dir @ Direction::Right => {
                let (x, y) = waypoint;
                waypoint = match rotate(dir, Direction::North, instr.units) {
                    Direction::North => (x, y),
                    Direction::East => (y, -x),
                    Direction::South => (-x, -y),
                    Direction::West => (-y, x),
                    _ => unreachable!(),
                }
            }
        }
    }

    (ship_loc.0).abs() + (ship_loc.1).abs()
}

fn move_with_dir(dir: Direction, x: isize, y: isize, units: isize) -> (isize, isize) {
    match dir {
        Direction::North => (x, y + units),
        Direction::South => (x, y - units),
        Direction::East => (x + units, y),
        Direction::West => (x - units, y),
        _ => unreachable!(),
    }
}

fn rotate(dir: Direction, cur: Direction, degrees: isize) -> Direction {
    match (cur as isize + if dir == Direction::Left { -1 } else { 1 } * degrees / 90).rem_euclid(4)
    {
        0 => Direction::North,
        1 => Direction::East,
        2 => Direction::South,
        3 => Direction::West,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
                "F10\n\
                    N3\n\
                    F7\n\
                    R90\n\
                    F11"
            )),
            25
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator(
                "F10\n\
                    N3\n\
                    F7\n\
                    R90\n\
                    F11"
            )),
            286
        )
    }
}
