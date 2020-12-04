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
    fn from(mut map: HashMap<String, String>) -> Self {
        Passport {
            byr: map.remove("byr"),
            iyr: map.remove("iyr"),
            eyr: map.remove("eyr"),
            hgt: map.remove("hgt"),
            hcl: map.remove("hcl"),
            ecl: map.remove("ecl"),
            pid: map.remove("pid"),
            cid: map.remove("cid"),
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
}
