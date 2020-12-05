#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut seats = input
        .lines()
        .map(|line| {
            line.chars().fold(0, |acc, c| {
                let binary = match c {
                    'F' | 'L' => 0,
                    'B' | 'R' => 1,
                    _ => panic!("Unparsable char!"),
                };
                (acc << 1) | binary
            })
        })
        .collect::<Vec<usize>>();
    seats.sort();
    seats
}

#[aoc(day5, part1)]
pub fn part_1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn part_2(input: &[usize]) -> usize {
    for idx in 0..(input.len() - 1) {
        if input[idx] + 1 != input[idx + 1] {
            return input[idx] + 1;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
        let input = input_generator(&input);
        assert_eq!(part_1(&input), 820);
    }
}
