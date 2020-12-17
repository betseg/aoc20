use itertools::Itertools;

const PRESIZE: usize = 25;
// const PRESIZE: usize = 5;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    input[(PRESIZE..input.len())
        .find(|&n| {
            !input[n - PRESIZE..n]
                .iter()
                .combinations(2)
                .map(|c| c.iter().copied().sum::<usize>())
                .any(|m| m == input[n])
        })
        .unwrap()]
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let weakness = solve_part1(input);

    for i in 0..input.len() {
        for j in i..input.len() {
            if input[i..j].iter().sum::<usize>() == weakness {
                let min = input[i..j].iter().min().unwrap();
                let max = input[i..j].iter().max().unwrap();
                return min + max;
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
        assert_eq!(
            solve_part1(&input_generator(
                "35\n\
                    20\n\
                    15\n\
                    25\n\
                    47\n\
                    40\n\
                    62\n\
                    55\n\
                    65\n\
                    95\n\
                    102\n\
                    117\n\
                    150\n\
                    182\n\
                    127\n\
                    219\n\
                    299\n\
                    277\n\
                    309\n\
                    576"
            )),
            127
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator(
                "35\n\
                        20\n\
                        15\n\
                        25\n\
                        47\n\
                        40\n\
                        62\n\
                        55\n\
                        65\n\
                        95\n\
                        102\n\
                        117\n\
                        150\n\
                        182\n\
                        127\n\
                        219\n\
                        299\n\
                        277\n\
                        309\n\
                        576"
            )),
            62
        );
    }
}
