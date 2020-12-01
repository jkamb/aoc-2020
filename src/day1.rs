#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split_ascii_whitespace()
        .map(|c| c.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    for num in input {
        for i in input {
            if i + num == 2020 {
                return i * num;
            }
        }
    }
    return 0;
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
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
