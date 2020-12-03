#[derive(Debug, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Tree,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<Tile>> {
    let mut field = Vec::new();
    for line in input.lines() {
        field.push(Vec::new());
        for tile in line.chars() {
            let len = field.len();
            field[len - 1].push(match tile {
                '.' => Tile::Empty,
                '#' => Tile::Tree,
                _ => unreachable!(),
            });
        }
    }
    field
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<Tile>]) -> usize {
    do_the_thing(input, 1, 3)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<Tile>]) -> usize {
    let slopes: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    slopes
        .iter()
        .map(|(y, x)| do_the_thing(input, *y, *x))
        .product()
}

fn do_the_thing(input: &[Vec<Tile>], y: usize, x: usize) -> usize {
    let mut trees = 0;
    let mut j = 0;
    for i in (0..input.len()).step_by(y) {
        if input[i][j] == Tile::Tree {
            trees += 1;
        }
        j += x;
        j %= input[0].len();
    }
    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "..##.......\n\
        #...#...#..\n\
        .#....#..#.\n\
        ..#.#...#.#\n\
        .#...##..#.\n\
        ..#.##.....\n\
        .#.#.#....#\n\
        .#........#\n\
        #.##...#...\n\
        #...##....#\n\
        .#..#...#.#";
        assert_eq!(solve_part1(&input_generator(input)), 7);
    }

    #[test]
    fn example2() {
        let input = "..##.......\n\
        #...#...#..\n\
        .#....#..#.\n\
        ..#.#...#.#\n\
        .#...##..#.\n\
        ..#.##.....\n\
        .#.#.#....#\n\
        .#........#\n\
        #.##...#...\n\
        #...##....#\n\
        .#..#...#.#";
        assert_eq!(solve_part2(&input_generator(input)), 336);
    }
}
