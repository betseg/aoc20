use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum State {
    Inactive,
    Active,
}

#[derive(Default)]
struct Field {
    field: HashMap<(isize, isize, isize, isize), State>,
    step: isize,
    initial_size: (isize, isize),
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Vec<State>> {
    let mut field = Vec::new();
    for l in input.lines() {
        field.push(Vec::new());
        for c in l.chars() {
            let len = field.len();
            field[len - 1].push(c.into());
        }
    }
    field
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &[Vec<State>]) -> usize {
    let mut field = Field::from(input);
    for _ in 0..6 {
        field = field.tick1();
    }
    field
        .field
        .iter()
        .filter(|(_, s)| **s == State::Active)
        .count()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &[Vec<State>]) -> usize {
    let mut field = Field::from(input);
    for _ in 0..6 {
        field = field.tick2();
    }
    field
        .field
        .iter()
        .filter(|(_, s)| **s == State::Active)
        .count()
}

impl Field {
    fn new() -> Self {
        Default::default()
    }

    fn tick1(self) -> Self {
        let mut new = Self {
            step: self.step + 1,
            initial_size: self.initial_size,
            field: HashMap::new(),
        };
        for z in -self.step - 1..=self.step + 1 {
            for y in -self.step - 1..self.initial_size.1 + self.step + 1 {
                for x in -self.step - 1..self.initial_size.0 + self.step + 1 {
                    let mut nbrs = 0;
                    for dz in -1..=1 {
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dx == 0 && dy == 0 && dz == 0 {
                                    continue;
                                }
                                if let Some(State::Active) =
                                    self.field.get(&(dx + x, dy + y, dz + z, 0))
                                {
                                    nbrs += 1;
                                }
                            }
                        }
                    }
                    new.field.insert(
                        (x, y, z, 0),
                        match (self.field.get(&(x, y, z, 0)), nbrs) {
                            // (None | Some(State::Inactive), 3) => State::Active,
                            // (Some(State::Active), 2 | 3) => State::Active,
                            // RFC #2535
                            (None, 3) | (Some(State::Inactive), 3) => State::Active,
                            (Some(State::Active), 2) | (Some(State::Active), 3) => State::Active,
                            _ => State::Inactive,
                        },
                    );
                }
            }
        }
        new
    }

    fn tick2(self) -> Self {
        let mut new = Self {
            step: self.step + 1,
            initial_size: self.initial_size,
            field: HashMap::new(),
        };
        for w in -self.step - 1..=self.step + 1 {
            for z in -self.step - 1..=self.step + 1 {
                for y in -self.step - 1..self.initial_size.1 + self.step + 1 {
                    for x in -self.step - 1..self.initial_size.0 + self.step + 1 {
                        let mut nbrs = 0;
                        for dw in -1..=1 {
                            for dz in -1..=1 {
                                for dy in -1..=1 {
                                    for dx in -1..=1 {
                                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                            continue;
                                        }
                                        if let Some(State::Active) =
                                            self.field.get(&(dx + x, dy + y, dz + z, dw + w))
                                        {
                                            nbrs += 1;
                                        }
                                    }
                                }
                            }
                        }
                        new.field.insert(
                            (x, y, z, w),
                            match (self.field.get(&(x, y, z, w)), nbrs) {
                                // (None | Some(State::Inactive), 3) => State::Active,
                                // (Some(State::Active), 2 | 3) => State::Active,
                                // RFC #2535
                                (None, 3) | (Some(State::Inactive), 3) => State::Active,
                                (Some(State::Active), 2) | (Some(State::Active), 3) => {
                                    State::Active
                                }
                                _ => State::Inactive,
                            },
                        );
                    }
                }
            }
        }
        new
    }
}

impl From<&[Vec<State>]> for Field {
    fn from(input: &[Vec<State>]) -> Self {
        let mut field = Field::new();
        field.initial_size = (input[0].len() as isize, input.len() as isize);
        for (j, l) in input.iter().enumerate() {
            for (i, t) in l.iter().enumerate() {
                field.field.insert((i as isize, j as isize, 0, 0), *t);
            }
        }
        field
    }
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Inactive,
            '#' => Self::Active,
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Inactive => ".",
            Self::Active => "#",
        })
    }
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for w in -self.step..=self.step {
            for z in -self.step..=self.step {
                f.write_fmt(format_args!("z={}, w={}\n", z, w))?;
                for y in -self.step..self.initial_size.1 + self.step {
                    for x in -self.step..self.initial_size.0 + self.step {
                        f.write_fmt(format_args!(
                            "{:?}",
                            self.field
                                .get(&(x, y, z, w))
                                .or(Some(&State::Inactive))
                                .unwrap(),
                        ))?;
                    }
                    f.write_str("\n")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
                ".#.\n\
                 ..#\n\
                 ###"
            )),
            112
        );
    }

    // takes a bit long
    // even longer than actual input???
    #[test]
    #[ignore]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator(
                ".#.\n\
                 ..#\n\
                 ###"
            )),
            848
        );
    }
}
