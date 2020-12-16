use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    op: Operation,
    arg: isize,
}

enum Execute {
    Halts(isize),
    Loops(isize),
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction {
            op: match &line[0..3] {
                "acc" => Operation::Acc,
                "jmp" => Operation::Jmp,
                "nop" => Operation::Nop,
                _ => unreachable!(),
            },
            arg: line[4..].parse().unwrap(),
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Instruction]) -> isize {
    match run(input) {
        Execute::Loops(n) => n,
        _ => unreachable!(),
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Instruction]) -> isize {
    let mut copied = input.to_owned();

    for i in 0..input.len() {
        match input[i].op {
            Operation::Jmp => copied[i].op = Operation::Nop,
            Operation::Nop => copied[i].op = Operation::Jmp,
            Operation::Acc => continue,
        }

        match run(&copied) {
            Execute::Halts(n) => return n,
            Execute::Loops(_) => match copied[i].op {
                Operation::Jmp => copied[i].op = Operation::Nop,
                Operation::Nop => copied[i].op = Operation::Jmp,
                Operation::Acc => unreachable!(),
            },
        }
    }

    unreachable!()
}

fn run(input: &[Instruction]) -> Execute {
    let mut acc = 0;
    let mut ip = 0;

    let mut seen = HashSet::new();

    loop {
        if seen.contains(&ip) {
            return Execute::Loops(acc);
        }
        if ip >= input.len() {
            return Execute::Halts(acc);
        }
        seen.insert(ip);

        match input[ip].op {
            Operation::Acc => {
                acc += input[ip].arg;
                ip += 1;
            }
            Operation::Jmp => ip = ip.wrapping_add(input[ip].arg as usize),
            Operation::Nop => ip += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
                "nop +0\n\
                    acc +1\n\
                    jmp +4\n\
                    acc +3\n\
                    jmp -3\n\
                    acc -99\n\
                    acc +1\n\
                    jmp -4\n\
                    acc +6"
            )),
            5
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator(
                "nop +0\n\
                    acc +1\n\
                    jmp +4\n\
                    acc +3\n\
                    jmp -3\n\
                    acc -99\n\
                    acc +1\n\
                    jmp -4\n\
                    acc +6"
            )),
            8
        );
    }
}
