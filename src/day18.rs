peg::parser! {
    grammar p1() for str {
        rule number() -> usize
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule solve() -> usize = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "*" y:@ { x * y }
            --
            n:number() { n }
            "(" e:solve() ")" { e }
        }
    }
}

peg::parser! {
    grammar p2() for str {
        rule number() -> usize
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule solve() -> usize = precedence!{
            x:(@) "*" y:@ { x * y }
            --
            x:(@) "+" y:@ { x + y }
            --
            n:number() { n }
            "(" e:solve() ")" { e }
        }
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.chars().filter(|&c| c != ' ').collect())
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    input.iter().filter_map(|l| p1::solve(l).ok()).sum()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    input.iter().filter_map(|l| p2::solve(l).ok()).sum()
}

/*use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Digit(usize),
    Operator(Operator),
    OpenParen,
    CloseParen,
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|&c| c != ' ')
                .map(|ch| match ch {
                    '0'..='9' => Token::Digit(ch.to_digit(10).unwrap() as usize),
                    '+' => Token::Operator(Operator::Add),
                    '*' => Token::Operator(Operator::Mul),
                    '(' => Token::OpenParen,
                    ')' => Token::CloseParen,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[Vec<Token>]) -> usize {
    input.iter().map(|l| process(l)).sum()
}

fn process(input: &[Token]) -> usize {
    let (mut res, mut i) = match input[0] {
        Token::Digit(n) => (n, 1),
        _ => {
            let skip = 1 + get_matching_paren_ix(&input[1..]);
            (process(&input[1..skip - 1]), skip)
        }
    };
    while i < input.len() - 1 {
        if input[i + 1] == Token::OpenParen {
            let skip = i + get_matching_paren_ix(&input[i + 2..]);
            let paren = process(&input[i + 2..=skip]);
            match input[i] {
                Token::Operator(Operator::Add) => res += paren,
                Token::Operator(Operator::Mul) => res *= paren,
                _ => unreachable!(),
            }
            i = skip;
        } else {
            match input[i] {
                Token::Operator(Operator::Add) => res += usize::from(input[i + 1]),
                Token::Operator(Operator::Mul) => res *= usize::from(input[i + 1]),
                _ => unreachable!(),
            }
        }
        i += 2;
    }
    res
}

fn get_matching_paren_ix(list: &[Token]) -> usize {
    let mut loop_v = 1;
    let mut skip = 0;
    while loop_v != 0 {
        loop_v +=
            ((list[skip] == Token::OpenParen) as i32) - ((list[skip] == Token::CloseParen) as i32);
        skip += 1;
    }
    skip
}

impl From<Token> for usize {
    fn from(tok: Token) -> Self {
        match tok {
            Token::Digit(i) => i,
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i = if let Token::Digit(i) = self {
            format!("{}", i)
        } else {
            "".into()
        };
        f.write_str(match self {
            Token::Digit(_) => &i,
            Token::Operator(Operator::Add) => "+",
            Token::Operator(Operator::Mul) => "*",
            Token::OpenParen => "(",
            Token::CloseParen => ")",
        })
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            solve_part1(&input_generator(
                "1 + 2 * 3 + 4 * 5 + 6\n\
                 1 + (2 * 3) + (4 * (5 + 6))\n\
                 2 * 3 + (4 * 5)\n\
                 5 + (8 * 3 + 9 + 3 * 4 * 3)\n\
                 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n\
                 ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            71 + 51 + 26 + 437 + 12240 + 13632
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_part2(&input_generator(
                "1 + 2 * 3 + 4 * 5 + 6\n\
                 1 + (2 * 3) + (4 * (5 + 6))\n\
                 2 * 3 + (4 * 5)\n\
                 5 + (8 * 3 + 9 + 3 * 4 * 3)\n\
                 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n\
                 ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            231 + 51 + 46 + 1445 + 669060 + 23340
        );
    }
}
