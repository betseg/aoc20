use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut input = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();
    input.push(0);
    input.sort_unstable();
    input.push(input[input.len() - 1] + 3);
    input
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let (ones, threes) = input.windows(2).fold((0, 0), |acc, w| {
        if w[1] - w[0] == 1 {
            (acc.0 + 1, acc.1)
        } else if w[1] - w[0] == 3 {
            (acc.0, acc.1 + 1)
        } else {
            acc
        }
    });
    ones * threes
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut map = HashMap::new();
    dp(input, 0, &mut map)
}

// thanks Jonathan Paulson
fn dp(input: &[usize], i: usize, mut map: &mut HashMap<usize, usize>) -> usize {
    if i == input.len() - 1 {
        return 1;
    }

    if map.contains_key(&i) {
        return map[&i];
    }

    let mut val = 0;
    for j in i + 1..input.len() {
        if input[j] - input[i] > 3 {
            break;
        }
        val += dp(input, j, &mut map);
    }

    map.insert(i, val);
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_1() {
        assert_eq!(
            solve_part1(&input_generator(
                "16\n\
                    10\n\
                    15\n\
                    5\n\
                    1\n\
                    11\n\
                    7\n\
                    19\n\
                    6\n\
                    12\n\
                    4"
            )),
            7 * 5
        );
    }

    #[test]
    fn example1_2() {
        assert_eq!(
            solve_part1(&input_generator(
                "28\n\
                    33\n\
                    18\n\
                    42\n\
                    31\n\
                    14\n\
                    46\n\
                    20\n\
                    48\n\
                    47\n\
                    24\n\
                    23\n\
                    49\n\
                    45\n\
                    19\n\
                    38\n\
                    39\n\
                    11\n\
                    1\n\
                    32\n\
                    25\n\
                    35\n\
                    8\n\
                    17\n\
                    7\n\
                    9\n\
                    4\n\
                    2\n\
                    34\n\
                    10\n\
                    3"
            )),
            22 * 10
        );
    }

    #[test]
    fn example2_1() {
        assert_eq!(
            solve_part2(&input_generator(
                "16\n\
                    10\n\
                    15\n\
                    5\n\
                    1\n\
                    11\n\
                    7\n\
                    19\n\
                    6\n\
                    12\n\
                    4"
            )),
            8
        );
    }

    #[test]
    fn example2_2() {
        assert_eq!(
            solve_part2(&input_generator(
                "28\n\
                    33\n\
                    18\n\
                    42\n\
                    31\n\
                    14\n\
                    46\n\
                    20\n\
                    48\n\
                    47\n\
                    24\n\
                    23\n\
                    49\n\
                    45\n\
                    19\n\
                    38\n\
                    39\n\
                    11\n\
                    1\n\
                    32\n\
                    25\n\
                    35\n\
                    8\n\
                    17\n\
                    7\n\
                    9\n\
                    4\n\
                    2\n\
                    34\n\
                    10\n\
                    3"
            )),
            19208
        );
    }
}
