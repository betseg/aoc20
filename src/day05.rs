#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|seat| {
            seat.chars().fold(0, |acc, bit| {
                acc << 1 | if ['B', 'R'].contains(&bit) { 1 } else { 0 }
            })
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input.iter().max().unwrap().clone()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    (8..1017)
        .filter(|&n| input.contains(&(n - 1)) && !input.contains(&n) && input.contains(&(n + 1)))
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples1() {
        assert_eq!(
            input_generator("FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL"),
            vec![357, 567, 119, 820]
        );
    }
}
