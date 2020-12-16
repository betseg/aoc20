use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Vertex {
    name: String,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Edge {
    from: Vertex,
    to: Vertex,
    value: usize,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<Vertex, HashSet<Edge>> {
    let mut map = HashMap::new();

    'lines: for line in input.lines() {
        let mut rules = line.split(" bags contain");
        let big = Vertex::new(rules.next().unwrap().into());

        for smol in rules.next().unwrap().strip_suffix('.').unwrap().split(',') {
            if smol == " no other bags" {
                map.insert(big, HashSet::new());
                continue 'lines;
            }
            let smol = smol.trim_end_matches('s').strip_suffix(" bag").unwrap();
            let value = smol.split(' ').nth(1).unwrap().parse().unwrap();
            let smol_v = Vertex::new(
                smol.trim_start_matches(|c: char| c.is_ascii_whitespace() || c.is_ascii_digit())
                    .into(),
            );

            map.entry(big.clone())
                .or_insert_with(HashSet::new)
                .insert(Edge::new(big.clone(), smol_v.clone(), value));

            map.entry(smol_v.clone())
                .or_insert_with(HashSet::new)
                .insert(Edge::new(big.clone(), smol_v, value));
        }
    }

    map
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<Vertex, HashSet<Edge>>) -> usize {
    input.keys().filter(|big| dfs(big, input)).count() // i know this can be optimized, idc
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<Vertex, HashSet<Edge>>) -> usize {
    let start = Vertex::new("shiny gold".into());
    bfs(&start, input) - 1
}

fn dfs(big: &Vertex, input: &HashMap<Vertex, HashSet<Edge>>) -> bool {
    let target = Vertex::new("shiny gold".into());
    if input[big].is_empty() {
        return false;
    }
    for edge in input[big].iter() {
        if edge.from != *big {
            continue;
        }
        if edge.to == target {
            return true;
        }
        if dfs(&edge.to, input) {
            return true;
        }
    }
    false
}

fn bfs(start: &Vertex, input: &HashMap<Vertex, HashSet<Edge>>) -> usize {
    let mut res = 1;
    for edge in input[start].iter() {
        if edge.from != *start {
            continue;
        }
        res += edge.value * bfs(&edge.to, input);
    }
    res
}

impl Vertex {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Edge {
    pub fn new(from: Vertex, to: Vertex, value: usize) -> Self {
        Self { from, to, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                    dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                    bright white bags contain 1 shiny gold bag.\n\
                    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                    dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                    faded blue bags contain no other bags.\n\
                    dotted black bags contain no other bags."
            )),
            4
        );
    }
    #[test]
    fn example2_1() {
        assert_eq!(
            solve_part2(&input_generator(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
                    dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
                    bright white bags contain 1 shiny gold bag.\n\
                    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
                    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
                    dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
                    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
                    faded blue bags contain no other bags.\n\
                    dotted black bags contain no other bags."
            )),
            32
        );
    }

    #[test]
    fn example2_2() {
        assert_eq!(
            solve_part2(&input_generator(
                "shiny gold bags contain 2 dark red bags.\n\
                    dark red bags contain 2 dark orange bags.\n\
                    dark orange bags contain 2 dark yellow bags.\n\
                    dark yellow bags contain 2 dark green bags.\n\
                    dark green bags contain 2 dark blue bags.\n\
                    dark blue bags contain 2 dark violet bags.\n\
                    dark violet bags contain no other bags."
            )),
            126
        );
    }
}
