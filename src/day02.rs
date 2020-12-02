use scan_fmt::scan_fmt;

#[derive(Debug)]
pub struct Password {
    lo: usize,
    hi: usize,
    letter: char,
    pw: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    let mut pws = Vec::new();
    for pw in input.lines() {
        if let Ok((lo, hi, letter, pw)) = scan_fmt!(pw, "{}-{} {}: {}", usize, usize, char, String)
        {
            pws.push(Password { lo, hi, letter, pw });
        }
    }
    pws
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Password]) -> usize {
    let mut n = 0;
    for pw in input {
        let m = pw.pw.chars().filter(|&c| c == pw.letter).count();
        if (pw.lo..=pw.hi).contains(&m) {
            n += 1;
        }
    }
    n
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Password]) -> usize {
    let mut n = 0;
    for pw in input {
        if pw.pw.chars().nth(pw.lo - 1).unwrap() == pw.letter
            && pw.pw.chars().nth(pw.hi - 1).unwrap() != pw.letter
            || pw.pw.chars().nth(pw.lo - 1).unwrap() != pw.letter
                && pw.pw.chars().nth(pw.hi - 1).unwrap() == pw.letter
        {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "1-3 a: abcde\n\
        1-3 b: cdefg\n\
        2-9 c: ccccccccc";
        assert_eq!(solve_part1(&input_generator(input)), 2);
    }

    #[test]
    fn example2() {
        let input = "1-3 a: abcde\n\
        1-3 b: cdefg\n\
        2-9 c: ccccccccc";
        assert_eq!(solve_part2(&input_generator(input)), 1);
    }
}
