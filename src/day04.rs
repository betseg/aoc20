#[derive(Debug)]
pub enum Height {
    Cm(Result<u8, std::num::ParseIntError>),
    In(Result<u8, std::num::ParseIntError>),
    Error,
}

#[derive(Debug)]
pub struct Passport {
    birth_year: Option<Result<u32, std::num::ParseIntError>>,
    issue_year: Option<Result<u32, std::num::ParseIntError>>,
    expr_year: Option<Result<u32, std::num::ParseIntError>>,
    height: Option<Height>,
    hair_color: Option<Option<u32>>,
    eye_color: Option<Result<(), ()>>,
    passport_id: Option<Option<u128>>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    let mut ps = Vec::new();
    for p in input.split("\n\n") {
        let mut pp = Passport::new();

        for field in p.split_whitespace() {
            match &field[..3] {
                "byr" => pp.birth_year = Some(field[4..].parse()),
                "iyr" => pp.issue_year = Some(field[4..].parse()),
                "eyr" => pp.expr_year = Some(field[4..].parse()),
                "hgt" => {
                    pp.height = Some(match &field[field.len() - 2..] {
                        "cm" => Height::Cm(field[4..field.len() - 2].parse()),
                        "in" => Height::In(field[4..field.len() - 2].parse()),
                        _ => Height::Error,
                    })
                }
                "hcl" => {
                    pp.hair_color = Some(if field.len() == 11 && &field[4..5] == "#" {
                        u32::from_str_radix(&field[5..], 16).ok()
                    } else {
                        None
                    })
                }
                "ecl" => {
                    pp.eye_color = Some(match &field[4..] {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(()),
                        _ => Err(()),
                    })
                }
                "pid" => {
                    pp.passport_id = Some(if field.len() == 13 {
                        field[4..].parse().ok()
                    } else {
                        None
                    })
                }
                "cid" => (),
                _ => unreachable!(),
            };
        }
        ps.push(pp);
    }
    ps
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Passport]) -> usize {
    input.iter().filter(|&p| p.is_valid_for_p1()).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Passport]) -> usize {
    input.iter().filter(|&p| p.is_valid_for_p2()).count()
}

impl Passport {
    fn new() -> Self {
        Self {
            birth_year: None,
            issue_year: None,
            expr_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
        }
    }

    fn is_valid_for_p1(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expr_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid_for_p2(&self) -> bool {
        self.is_valid_for_p1()
            && self.byr_valid()
            && self.iyr_valid()
            && self.eyr_valid()
            && self.hgt_valid()
            && self.hair_color.unwrap().is_some()
            && self.eye_color == Some(Ok(()))
            && self.passport_id.unwrap().is_some()
    }

    fn byr_valid(&self) -> bool {
        matches!(self.birth_year, Some(Ok(1920..=2002)))
    }

    fn iyr_valid(&self) -> bool {
        matches!(self.issue_year, Some(Ok(2010..=2020)))
    }

    fn eyr_valid(&self) -> bool {
        matches!(self.expr_year, Some(Ok(2020..=2030)))
    }

    fn hgt_valid(&self) -> bool {
        match self.height {
            Some(Height::Cm(Ok(cm))) => (150..=193).contains(&cm),
            Some(Height::In(Ok(inch))) => (59..=76).contains(&inch),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
        byr:1937 iyr:2017 cid:147 hgt:183cm\n\
        \n\
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
        hcl:#cfa07d byr:1929\n\
        \n\
        hcl:#ae17e1 iyr:2013\n\
        eyr:2024\n\
        ecl:brn pid:760753108 byr:1931\n\
        hgt:179cm\n\
        \n\
        hcl:#cfa07d eyr:2025 pid:166559648\n\
        iyr:2011 ecl:brn hgt:59in";
        assert_eq!(solve_part1(&input_generator(input)), 2);
    }

    #[test]
    fn invalid_exs() {
        let input = "eyr:1972 cid:100\n\
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
        \n\
        iyr:2019\n\
        hcl:#602927 eyr:1967 hgt:170cm\n\
        ecl:grn pid:012533040 byr:1946\n\
        \n\
        hcl:dab227 iyr:2012\n\
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
        \n\
        hgt:59cm ecl:zzz\n\
        eyr:2038 hcl:74454a iyr:2023\n\
        pid:3556412378 byr:2007";
        assert_eq!(solve_part2(&input_generator(input)), 0);
    }

    #[test]
    fn valid_exs() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
        hcl:#623a2f\n\
        \n\
        eyr:2029 ecl:blu cid:129 byr:1989\n\
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
        \n\
        hcl:#888785\n\
        hgt:164cm byr:2001 iyr:2015 cid:88\n\
        pid:545766238 ecl:hzl\n\
        eyr:2022\n\
        \n\
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(solve_part2(&input_generator(input)), 4);
    }
}
