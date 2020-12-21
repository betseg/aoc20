use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::{collections::HashMap, fmt, num::ParseIntError, ops::BitOr, str::FromStr};

#[derive(Copy, Clone)]
pub struct Mask {
    mask: [Option<bool>; 36],
}

#[derive(Debug)]
pub enum Instruction {
    Mask(Mask),
    Write(u64, u64),
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    let mut res = Vec::new();
    for line in input.lines() {
        if let Ok(mask) = scan_fmt!(line, "mask = {}", Mask) {
            res.push(Instruction::Mask(mask));
        } else if let Ok((addr, val)) = scan_fmt!(line, "mem[{}] = {}", u64, u64) {
            res.push(Instruction::Write(addr, val));
        }
    }
    res
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Instruction]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = Mask::new();
    for instr in input {
        match instr {
            Instruction::Mask(new_mask) => mask = *new_mask,
            Instruction::Write(addr, val) => *mem.entry(addr).or_default() = *val | mask,
        }
    }
    mem.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Instruction]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask = Mask::new();
    for instr in input {
        match instr {
            Instruction::Mask(new_mask) => mask = *new_mask,
            Instruction::Write(addr, val) => {
                for addr in decode(mask | *addr).iter() {
                    mem.insert(*addr, *val);
                }
            }
        }
    }
    mem.values().sum()
}

impl Mask {
    fn new() -> Self {
        Self { mask: [None; 36] }
    }
}

impl BitOr<Mask> for u64 {
    type Output = u64;

    // bruh what
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn bitor(self, rhs: Mask) -> Self::Output {
        rhs.mask
            .iter()
            .enumerate()
            .map(|(ix, mask)| {
                (match mask {
                    Some(true) => 1,
                    Some(false) => 0,
                    None => self >> (35 - ix),
                } % 2)
                    * 2u64.pow((35 - ix) as u32)
            })
            .sum()
    }
}

impl BitOr<u64> for Mask {
    type Output = Mask;

    fn bitor(self, rhs: u64) -> Self::Output {
        let mut res = Mask::new();
        for i in 0..36 {
            res.mask[i] = match self.mask[i] {
                Some(false) => Some((rhs >> (35 - i)) % 2 == 1),
                other => other,
            };
        }
        res
    }
}

fn decode(mask: Mask) -> Vec<u64> {
    let vars = mask.mask.iter().filter(|b| b.is_none()).count();
    let mut addrs = Vec::new();
    for prod in (0..vars)
        .map(|_| [false, true].iter())
        .multi_cartesian_product()
    {
        let mut addr = 0;
        let mut var_bits = prod.iter();
        for i in 0..36 {
            addr += 2u64.pow(35 - i as u32)
                * mask.mask[i]
                    .or_else(|| Some(**var_bits.next().unwrap()))
                    .unwrap() as u64;
        }
        addrs.push(addr);
    }
    addrs
}

impl FromStr for Mask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new = Mask::new();
        for (ix, bit) in s.chars().enumerate() {
            new.mask[ix] = match bit {
                '1' => Some(true),
                '0' => Some(false),
                'X' => None,
                _ => unreachable!(),
            }
        }
        Ok(new)
    }
}

impl fmt::Debug for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in self.mask.iter() {
            match bit {
                Some(true) => f.write_str("1")?,
                Some(false) => f.write_str("0")?,
                None => f.write_str("X")?,
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
                "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
                    mem[8] = 11\n\
                    mem[7] = 101\n\
                    mem[8] = 0"
            )),
            165
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator(
                "mask = 000000000000000000000000000000X1001X\n\
                    mem[42] = 100\n\
                    mask = 00000000000000000000000000000000X0XX\n\
                    mem[26] = 1"
            )),
            208
        );
    }
}
