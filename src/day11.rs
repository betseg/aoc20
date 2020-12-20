//
// DO NOT READ
// disgusting code
// I'm not proud
//
// for your own good
//

use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Tile {
    Empty,
    Occupied,
    Floor,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Tile>> {
    let res = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| match ch {
                    'L' => Tile::Empty,
                    '#' => Tile::Occupied,
                    '.' | '\n' => Tile::Floor,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    res
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Vec<Tile>]) -> usize {
    let mut prev = 0;
    let mut input = tick1(&input);

    while prev
        != input
            .iter()
            .flatten()
            .filter(|&&width| width == Tile::Occupied)
            .count()
    {
        prev = input
            .iter()
            .flatten()
            .filter(|&&width| width == Tile::Occupied)
            .count();
        input = tick1(&input);
    }
    prev
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Vec<Tile>]) -> usize {
    let mut prev = 0;
    let mut input = tick2(&input);

    while prev
        != input
            .iter()
            .flatten()
            .filter(|&&width| width == Tile::Occupied)
            .count()
    {
        prev = input
            .iter()
            .flatten()
            .filter(|&&width| width == Tile::Occupied)
            .count();
        input = tick2(&input);
    }
    prev
}

fn tick1(input: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut next = input.to_owned();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let mut neighbors = 0;
            for dy in 0..=2 {
                for dx in 0..=2 {
                    if (dy == 1 && dx == 1) || (y == 0 && dy == 0) || (x == 0 && dx == 0) {
                        continue;
                    }
                    if y + dy - 1 < input.len()
                        && x + dx - 1 < input[0].len()
                        && input[(y + dy - 1)][(x + dx - 1)] == Tile::Occupied
                    {
                        neighbors += 1;
                    } else {
                        continue;
                    }
                }
            }

            let cur_tile = input[y][x];
            if cur_tile == Tile::Empty && neighbors == 0 {
                next[y][x] = Tile::Occupied;
            } else if cur_tile == Tile::Occupied && neighbors >= 4 {
                next[y][x] = Tile::Empty;
            }
        }
    }

    next
}

fn tick2(input: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut next = input.to_owned();

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let mut checked = [None; 9];
            for mul in 1.. {
                if checked.iter().filter(|&&t| t == None).count() == 1 {
                    break;
                }
                for dy in 0..=2 {
                    'inner: for dx in 0..=2 {
                        if dy == 1 && dx == 1 || checked[dy * 3 + dx].is_some() {
                            continue;
                        } else if (y == 0 && dy == 0) || (x == 0 && dx == 0) {
                            checked[dy * 3 + dx] = Some(Tile::Empty);
                            continue 'inner;
                        }
                        let mdy = (dy as isize - 1) * mul;
                        let mdx = (dx as isize - 1) * mul;
                        if y as isize + mdy >= 0
                            && x as isize + mdx >= 0
                            && y.overflowing_add(mdy as usize).0 < input.len()
                            && x.overflowing_add(mdx as usize).0 < input[0].len()
                        {
                            match input[y.overflowing_add(mdy as usize).0]
                                [x.overflowing_add(mdx as usize).0]
                            {
                                Tile::Occupied => checked[dy * 3 + dx] = Some(Tile::Occupied),
                                Tile::Empty => checked[dy * 3 + dx] = Some(Tile::Empty),
                                Tile::Floor => (),
                            }
                        } else {
                            checked[dy * 3 + dx] = Some(Tile::Empty);
                            continue 'inner;
                        }
                    }
                }
            }

            let cur_tile = input[y][x];
            if cur_tile == Tile::Empty
                && checked
                    .iter()
                    .filter(|&&t| t == Some(Tile::Occupied))
                    .count()
                    == 0
            {
                next[y][x] = Tile::Occupied;
            } else if cur_tile == Tile::Occupied
                && checked
                    .iter()
                    .filter(|&&t| t == Some(Tile::Occupied))
                    .count()
                    >= 5
            {
                next[y][x] = Tile::Empty;
            }
        }
    }

    next
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => 'L',
                Tile::Occupied => '#',
                Tile::Floor => '.',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
                "L.LL.LL.LL\n\
                    LLLLLLL.LL\n\
                    L.L.L..L..\n\
                    LLLL.LL.LL\n\
                    L.LL.LL.LL\n\
                    L.LLLLL.LL\n\
                    ..L.L.....\n\
                    LLLLLLLLLL\n\
                    L.LLLLLL.L\n\
                    L.LLLLL.LL"
            )),
            37
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator(
                "L.LL.LL.LL\n\
                    LLLLLLL.LL\n\
                    L.L.L..L..\n\
                    LLLL.LL.LL\n\
                    L.LL.LL.LL\n\
                    L.LLLLL.LL\n\
                    ..L.L.....\n\
                    LLLLLLLLLL\n\
                    L.LLLLLL.L\n\
                    L.LLLLL.LL"
            )),
            26
        );
    }
}
