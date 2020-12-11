use itertools::Itertools;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|c| c.parse::<usize>().unwrap()).collect()
}

fn invalid_number(input: &[usize], preamble_size: usize) -> usize {
    let windows: Vec<_> = input
        .windows(preamble_size)
        .zip(input.iter().skip(preamble_size))
        .collect();

    for (windows, value) in windows {
        if !windows
            .iter()
            .tuple_combinations()
            .any(|(a, b)| a + b == *value)
        {
            return *value;
        }
    }
    panic!("Could not find value")
}

fn sliding_window(input: &[usize], invalid_number: usize) -> usize {
    for index in 0..input.len() {
        let mut set = input
            .windows(2 + index)
            .filter(|w| w.iter().sum::<usize>() == invalid_number)
            .flatten()
            .collect::<Vec<_>>();
        if !set.is_empty() {
            set.sort();
            return *set.first().unwrap() + *set.last().unwrap();
        }
    }
    panic!("Failed to find weakness in sliding window")
}

#[aoc(day9, part1)]
pub fn part_1(input: &[usize]) -> usize {
    invalid_number(&input, 25)
}

#[aoc(day9, part2)]
pub fn part_2(input: &[usize]) -> usize {
    sliding_window(&input, invalid_number(&input, 25))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let input = input_generator(input);
        assert_eq!(invalid_number(&input, 5), 127);
    }

    #[test]
    fn test_part2() {
        let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let input = input_generator(input);
        assert_eq!(sliding_window(&input, invalid_number(&input, 5)), 62)
    }
}
