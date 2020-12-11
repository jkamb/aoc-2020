#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut adapters: Vec<usize> = input
        .lines()
        .map(|c| c.parse::<usize>().unwrap())
        .chain(std::iter::once(0))
        .collect();
    adapters.sort();
    adapters
}

#[aoc(day10, part2)]
pub fn part_1(input: &[usize]) -> usize {
    let mut one_jolt = 0;
    let mut three_jolt = 0;
    for (index, adapter) in input.iter().enumerate() {
        if index == input.len() - 1 {
            three_jolt += 1;
            break;
        }

        let diff = input[index + 1] - adapter;
        println!("{:?} {}", adapter, diff);
        match diff {
            1 => one_jolt += 1,
            3 => three_jolt += 1,
            _ => (),
        }
    }
    println!("{} {}", one_jolt, three_jolt);
    one_jolt * three_jolt
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let input = input_generator(input);
        assert_eq!(part_1(&input), 220);
    }
}
