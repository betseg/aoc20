use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    do_the_thing(input, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    do_the_thing(input, 30_000_000)
}

pub fn do_the_thing(input: &[usize], n: usize) -> usize {
    let mut map: HashMap<_, _> = input
        .iter()
        .copied()
        .enumerate()
        .map(|(ix, n)| (n, (None, ix)))
        .collect();
    let mut last = input[input.len() - 1];

    for i in input.len()..n {
        let entry = map.entry(last).or_insert((None, i));

        let new = entry.1 - if let Some(n) = entry.0 { n } else { entry.1 };

        map.insert(
            new,
            if let Some(entry) = map.get(&new) {
                (Some(entry.1), i)
            } else {
                (None, i)
            },
        );

        last = new;
    }

    *map.iter().find(|(_, (_, g))| *g == n - 1).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples1() {
        assert_eq!(solve_part1(&input_generator("0,3,6")), 436);
        assert_eq!(solve_part1(&input_generator("1,3,2")), 1);
        assert_eq!(solve_part1(&input_generator("2,1,3")), 10);
        assert_eq!(solve_part1(&input_generator("1,2,3")), 27);
        assert_eq!(solve_part1(&input_generator("2,3,1")), 78);
        assert_eq!(solve_part1(&input_generator("3,2,1")), 438);
        assert_eq!(solve_part1(&input_generator("3,1,2")), 1836);
    }

    // takes long
    #[test]
    #[ignore]
    fn examples2() {
        assert_eq!(solve_part2(&input_generator("0,3,6")), 175594);
        assert_eq!(solve_part2(&input_generator("1,3,2")), 2578);
        assert_eq!(solve_part2(&input_generator("2,1,3")), 3544142);
        assert_eq!(solve_part2(&input_generator("1,2,3")), 261214);
        assert_eq!(solve_part2(&input_generator("2,3,1")), 6895259);
        assert_eq!(solve_part2(&input_generator("3,2,1")), 18);
        assert_eq!(solve_part2(&input_generator("3,1,2")), 362);
    }
}
