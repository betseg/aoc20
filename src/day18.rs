peg::parser! {
    grammar p1() for str {
        rule number() -> usize
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule solve() -> usize = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "*" y:@ { x * y }
            --
            n:number() { n }
            "(" e:solve() ")" { e }
        }
    }
}

peg::parser! {
    grammar p2() for str {
        rule number() -> usize
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule solve() -> usize = precedence!{
            x:(@) "*" y:@ { x * y }
            --
            x:(@) "+" y:@ { x + y }
            --
            n:number() { n }
            "(" e:solve() ")" { e }
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.chars().filter(|&c| c != ' ').collect())
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    input.iter().filter_map(|l| p1::solve(l).ok()).sum()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    input.iter().filter_map(|l| p2::solve(l).ok()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
                "1 + 2 * 3 + 4 * 5 + 6\n\
                 1 + (2 * 3) + (4 * (5 + 6))\n\
                 2 * 3 + (4 * 5)\n\
                 5 + (8 * 3 + 9 + 3 * 4 * 3)\n\
                 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n\
                 ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            71 + 51 + 26 + 437 + 12240 + 13632
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator(
                "1 + 2 * 3 + 4 * 5 + 6\n\
                 1 + (2 * 3) + (4 * (5 + 6))\n\
                 2 * 3 + (4 * 5)\n\
                 5 + (8 * 3 + 9 + 3 * 4 * 3)\n\
                 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n\
                 ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            231 + 51 + 46 + 1445 + 669060 + 23340
        );
    }
}
