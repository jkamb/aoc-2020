fn tree_count(input: &str, x: usize, y: usize) -> usize {
    input
        .lines()
        .step_by(y)
        .enumerate()
        .filter(|&(i, line)| line.as_bytes()[i * x % line.len()] == b'#')
        .count()
}

#[aoc(day3, part1)]
pub fn part_1(input: &str) -> usize {
    tree_count(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(x, y)| tree_count(input, x, y))
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(part_2(&input), 336);
    }
}
