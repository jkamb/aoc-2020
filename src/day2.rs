#[derive(Debug)]
struct Policy {
    c: char,
    max: usize,
    min: usize,
}

#[derive(Debug)]
pub struct Password {
    policy: Policy,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    let lines = input.lines();
    lines
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let policy_raw: Vec<&str> = parts[0].split(" ").collect();
            let min_max: Vec<&str> = policy_raw[0].split("-").collect();
            let character = policy_raw[1].chars().nth(0).unwrap();
            let policy = Policy {
                c: character.to_owned(),
                min: min_max[0].parse().unwrap(),
                max: min_max[1].parse().unwrap(),
            };
            Password {
                policy,
                password: parts[1].trim().to_string(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part_1(input: &[Password]) -> usize {
    input
        .iter()
        .filter(|pwd| {
            let count = pwd.password.chars().filter(|&c| c == pwd.policy.c).count();
            count >= pwd.policy.min && count <= pwd.policy.max
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part_2(input: &[Password]) -> usize {
    input
        .iter()
        .filter(|pwd| {
            pwd.password
                .char_indices()
                .filter(|(i, _)| i + 1 == pwd.policy.min || i + 1 == pwd.policy.max)
                .filter(|&(_, c)| c == pwd.policy.c)
                .count()
                == 1
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let input = input_generator(input);
        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let input = input_generator(input);
        assert_eq!(part_2(&input), 1);
    }
}
