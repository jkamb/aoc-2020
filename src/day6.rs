use std::collections::HashMap;
use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| {
            s.chars()
                .filter(|&c| c != '\n')
                .fold(HashSet::new(), |mut set, c| {
                    set.insert(c);
                    set
                })
                .iter()
                .count()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let group_count = group.split("\n").count();
            group
                .split("\n")
                .fold(HashMap::new(), |mut collection, person| {
                    person.chars().for_each(|c| {
                        *collection.entry(c).or_insert(0) += 1;
                    });
                    collection
                })
                .iter()
                .fold(
                    0,
                    |acc, (_, &count)| if count == group_count { acc + 1 } else { acc },
                )
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(part_1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(part_2(&input), 6);
    }
}
