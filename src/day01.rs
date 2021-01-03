#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    for a in input {
        for b in input {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    for a in input {
        for b in input {
            for c in input {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(solve_part1(&input_generator(input)), 514579);
    }

    #[test]
    fn example2() {
        let input = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(solve_part2(&input_generator(input)), 241861950);
    }
}
