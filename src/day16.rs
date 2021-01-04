use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq)]
pub struct Rule {
    name: String,
    lo: RangeInclusive<usize>,
    hi: RangeInclusive<usize>,
}

#[derive(Default)]
pub struct Input {
    rules: Vec<Rule>,
    mine: Vec<usize>,
    others: Vec<Vec<usize>>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let mut res: Input = Default::default();

    while let Ok((name, ll, lh, hl, hh)) = scan_fmt!(
        lines.next().unwrap(),
        "{[a-z ]}: {}-{} or {}-{}",
        String,
        usize,
        usize,
        usize,
        usize
    ) {
        res.rules.push(Rule {
            name,
            lo: ll..=lh,
            hi: hl..=hh,
        });
    }

    lines.next();

    res.mine = lines
        .next()
        .map(|s| s.split(',').map(str::parse).map(Result::unwrap))
        .unwrap()
        .collect();

    lines.next();
    lines.next();

    for line in lines {
        res.others.push(
            line.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
        );
    }
    res
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> usize {
    errors(input).0
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Input) -> usize {
    my_ticket(input)
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .fold(1, |acc, (_, n)| acc * n)
}

fn errors(input: &Input) -> (usize, Vec<Vec<usize>>) {
    let mut errors = 0;
    let mut valids = Vec::new();
    'outer: for ticket in &input.others {
        for field in ticket {
            if input.rules.iter().all(|r| !rule_holds(*field, r)) {
                errors += field;
                continue 'outer;
            }
        }
        valids.push(ticket.clone());
    }
    (errors, valids)
}

fn ruleset(input: &Input) -> Vec<&Rule> {
    let _valids = errors(input).1;
    let mut _found = Vec::<usize>::new();
    todo!()
    //    found.iter().map(|&i| &input.rules[i]).collect()
}

pub fn my_ticket(input: &Input) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for (i, r) in ruleset(input).iter().enumerate() {
        map.insert(r.name.to_owned(), input.mine[i]);
    }
    map
}

fn rule_holds(n: usize, rule: &Rule) -> bool {
    rule.lo.contains(&n) || rule.hi.contains(&n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
                "class: 1-3 or 5-7\n\
                    row: 6-11 or 33-44\n\
                    seat: 13-40 or 45-50\n\
                    \n\
                    your ticket:\n\
                    7,1,14\n\
                    \n\
                    nearby tickets:\n\
                    7,3,47\n\
                    40,4,50\n\
                    55,2,20\n\
                    38,6,12"
            )),
            4 + 55 + 12
        );
    }

    // #[test]
    // fn example2() {
    //     let res = {
    //         let mut map = HashMap::new();
    //         map.insert("class".into(), 12);
    //         map.insert("row".into(), 11);
    //         map.insert("seat".into(), 13);
    //         map
    //     };

    //     assert_eq!(
    //         my_ticket(&input_generator(
    //             "class: 0-1 or 4-19\n\
    //                 row: 0-5 or 8-19\n\
    //                 seat: 0-13 or 16-19\n\
    //                 \n\
    //                 your ticket:\n\
    //                 11,12,13\n\
    //                 \n\
    //                 nearby tickets:\n\
    //                 3,9,18\n\
    //                 15,1,5\n\
    //                 5,14,9"
    //         )),
    //         res
    //     );
    // }
}
