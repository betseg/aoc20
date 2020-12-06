use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    let mut groups = Vec::new();
    for group in input.split("\n\n") {
        let mut gp = Vec::new();
        for ch in group.chars() {
            gp.push(ch);
        }
        groups.push(gp);
    }
    groups
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> usize {
    let mut val = 0;
    for group in input {
        let mut map = HashSet::new();
        for person in group.split(|&c| c == '\n') {
            for ch in person {
                map.insert(ch);
            }
        }
        val += map.len();
    }
    val
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> usize {
    let mut val = 0;
    for group in input {
        let mut maps = Vec::new();
        for person in group.split(|&c| c == '\n') {
            let mut map = HashSet::new();
            for ch in person {
                map.insert(ch);
            }
            maps.push(map);
        }
        val += maps.iter().fold(maps[0].clone(), |acc, m| &acc & m).len();
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples1() {
        assert_eq!(solve_part1(&input_generator("abcx\nabcy\nabcz")), 6);
        assert_eq!(
            solve_part1(&input_generator(
                "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"
            )),
            11
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator(
                "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"
            )),
            6
        );
    }
}
