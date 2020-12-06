#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut seats = Vec::new();
    for seat in input.lines() {
        let mut n = 0;
        for bit in seat.chars() {
            if ['B', 'R'].contains(&bit) {
                n |= 1;
            }
            n <<= 1;
        }
        n >>= 1;
        seats.push(n);
    }
    seats
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
