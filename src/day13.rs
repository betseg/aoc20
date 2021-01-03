#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (isize, Vec<Option<isize>>) {
    (
        input.lines().next().unwrap().parse().unwrap(),
        input
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(str::parse)
            .map(Result::ok)
            .collect(),
    )
}

#[aoc(day13, part1)]
pub fn solve_part1((est, input): &(isize, Vec<Option<isize>>)) -> isize {
    input
        .iter()
        .filter_map(|&b| b)
        .map(|b| (b - est % b, b))
        .min_by_key(|b| b.0)
        .map(|b| b.0 * b.1)
        .unwrap()
}

#[aoc(day13, part2)]
pub fn solve_part2((_est, input): &(isize, Vec<Option<isize>>)) -> isize {
    let buses = input
        .iter()
        .enumerate()
        .filter(|(_ix, b)| b.is_some())
        .map(|(ix, b)| {
            let b = b.unwrap();
            (b, ix as isize)
        })
        .collect::<Vec<_>>();
    let mul = buses.iter().map(|b| b.0).product::<isize>();
    mul - buses
        .iter()
        .map(|(a, b)| {
            let (bigm, _m) = bezout(mul / a, *a);
            (b * bigm * mul / a).rem_euclid(mul)
        })
        .sum::<isize>()
        % mul
}

#[allow(clippy::many_single_char_names)]
fn bezout(a: isize, b: isize) -> (isize, isize) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let q = old_r / r;
        // (old_r, r) = (r, old_r - q * r);
        // (old_s, s) = (s, old_s - q * s);
        // (old_t, t) = (r, old_t - q * t);
        // RFC #372
        let (a, b) = (r, old_r - q * r);
        old_r = a;
        r = b;
        let (a, b) = (s, old_s - q * s);
        old_s = a;
        s = b;
        let (a, b) = (t, old_t - q * t);
        old_t = a;
        t = b;
    }

    (old_s, old_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
                "939\n\
                    7,13,x,x,59,x,31,19"
            )),
            295
        );
    }

    #[test]
    fn examples2() {
        assert_eq!(solve_part2(&input_generator("0\n17,x,13,19")), 3417);
        assert_eq!(solve_part2(&input_generator("0\n67,7,59,61")), 754018);
        assert_eq!(solve_part2(&input_generator("0\n67,x,7,59,61")), 779210);
        assert_eq!(
            solve_part2(&input_generator("939\n7,13,x,x,59,x,31,19")),
            1068781
        );
        assert_eq!(solve_part2(&input_generator("0\n67,7,x,59,61")), 1261476);
        assert_eq!(
            solve_part2(&input_generator("0\n1789,37,47,1889")),
            1202161486
        );
    }
}
