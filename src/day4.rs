use std::collections::HashMap;
#[derive(Debug)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn from(map: HashMap<String, String>) -> Self {
        Passport {
            byr: map.get("byr").map(|s| s.clone()),
            iyr: map.get("iyr").map(|s| s.clone()),
            eyr: map.get("eyr").map(|s| s.clone()),
            hgt: map.get("hgt").map(|s| s.clone()),
            hcl: map.get("hcl").map(|s| s.clone()),
            ecl: map.get("ecl").map(|s| s.clone()),
            pid: map.get("pid").map(|s| s.clone()),
            cid: map.get("cid").map(|s| s.clone()),
        }
    }

    fn valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn validate_hgt(&self) -> bool {
        match self.hgt {
            Some(ref hgt) => hgt[..hgt.len() - 2].parse::<u32>().map_or(false, |val| {
                match &hgt[hgt.len() - 2..] {
                    "in" => val >= 59 || val <= 76,
                    "cm" => val >= 150 || val <= 193,
                    _ => false,
                }
            }),
            _ => false,
        }
    }

    fn validate_ecl(&self) -> bool {
        match self.ecl {
            Some(ref ecl) => {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str())
            }
            _ => false,
        }
    }

    fn validate_hcl(&self) -> bool {
        match self.hcl {
            Some(ref hcl) => {
                hcl.len() == 7
                    && hcl.chars().nth(0) == Some('#')
                    && hcl.chars().skip(1).all(char::is_alphanumeric)
            }
            _ => false,
        }
    }

    fn validate_pid(&self) -> bool {
        match self.pid {
            Some(ref pid) => pid.len() == 9 && pid.chars().all(char::is_numeric),
            _ => false,
        }
    }

    fn validate_byr(&self) -> bool {
        match self.byr {
            Some(ref byr) => {
                byr.len() == 4
                    && byr
                        .parse::<u32>()
                        .map_or(false, |val| val >= 1920 && val <= 2002)
            }
            _ => false,
        }
    }

    fn validate_iyr(&self) -> bool {
        match self.iyr {
            Some(ref iyr) => {
                iyr.len() == 4
                    && iyr
                        .parse::<u32>()
                        .map_or(false, |val| val >= 2010 && val <= 2020)
            }
            _ => false,
        }
    }

    fn validate_eyr(&self) -> bool {
        match self.eyr {
            Some(ref eyr) => {
                eyr.len() == 4
                    && eyr
                        .parse::<u32>()
                        .map_or(false, |val| val >= 2020 && val <= 2030)
            }
            _ => false,
        }
    }

    fn extended_validation(&self) -> bool {
        self.valid()
            && self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_ecl()
            && self.validate_hcl()
            && self.validate_pid()
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|passport| {
            let parsed = passport
                .split_whitespace()
                .map(|p| {
                    let parts: Vec<&str> = p.split(":").collect();
                    (parts[0].to_owned(), parts[1].to_owned())
                })
                .collect::<HashMap<_, _>>();
            Passport::from(parsed)
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part_1(input: &[Passport]) -> usize {
    input.iter().filter(|passport| passport.valid()).count()
}

#[aoc(day4, part2)]
pub fn part_2(input: &[Passport]) -> usize {
    input
        .iter()
        .inspect(|pass| println!("Before filter {:?}", pass))
        .filter(|passport| passport.extended_validation())
        .inspect(|pass| println!("After filter {:?}", pass))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let input = input_generator(input);
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_invalid() {
        let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let input = input_generator(input);
        assert_eq!(part_2(&input), 0);
    }

    #[test]
    fn test_valid() {
        let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let input = input_generator(input);
        assert_eq!(part_2(&input), 4);
    }
}
