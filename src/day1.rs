use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|c| c.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part_1(input: &[usize]) -> usize {
    for num in input {
        for i in input {
            if i + num == 2020 {
                return i * num;
            }
        }
    }
    return 0;
}

#[aoc(day1, part1, iter)]
pub fn part_1_iterator(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| if a + b == 2020 { Some(a * b) } else { None })
        .next()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part_2(input: &[usize]) -> usize {
    for num in input {
        for j in input {
            for i in input {
                if j + i + num == 2020 {
                    return j * i * num;
                }
            }
        }
    }
    return 0;
}

#[aoc(day1, part2, iter)]
pub fn part_2_iterator(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b, c)| {
            if a + b + c == 2020 {
                Some(a * b * c)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}
