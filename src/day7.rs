use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<(u32, String)>> {
    input
        .lines()
        .map(|c| {
            let find_str = "contain";
            let index = c.find(find_str).unwrap();
            let main_bag = c[..index].split("bag").next().unwrap().trim().to_owned();
            let contained_bags: Vec<(u32, String)> = c[index + find_str.len()..]
                .split(",")
                .filter_map(|s| {
                    if s.trim() == "no other bags." {
                        return None;
                    }

                    let bag_type = s.split("bag").next().unwrap().trim();
                    let bag_num: u32 = bag_type
                        .chars()
                        .take(1)
                        .map(|c| c.to_digit(10).expect("Char was not a digit?!"))
                        .next()
                        .unwrap();
                    Some((bag_num, bag_type[2..].trim().to_owned()))
                })
                .collect();
            (main_bag, contained_bags)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part_1(input: &HashMap<String, Vec<(u32, String)>>) -> usize {
    let mut bags_to_consider = VecDeque::new();
    bags_to_consider.push_back("shiny gold");
    let mut seen = HashSet::new();
    while let Some(current_bag) = bags_to_consider.pop_front() {
        for (main_bag, bags) in input {
            for (_, bag) in bags {
                if bag.contains(current_bag) {
                    println!("{} has a {}", main_bag, current_bag);
                    seen.insert(main_bag);
                    bags_to_consider.push_back(main_bag);
                }
            }
        }
    }
    println!("{:?}", seen);
    seen.len()
}

#[aoc(day7, part2)]
pub fn part_2(input: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    let mut bags_to_count = VecDeque::new();
    bags_to_count.push_back(("shiny gold", 1));
    let mut count = 0;
    while let Some((current_bag, num_of_bags)) = bags_to_count.pop_front() {
        count += num_of_bags;
        println!("{} has {} bags, total: {}", current_bag, num_of_bags, count);
        if let Some(bag) = input.get(current_bag) {
            for (number_of_bags, bag_name) in bag {
                println!("{} adds {} {} bags", current_bag, bag_name, number_of_bags);
                bags_to_count.push_back((bag_name, number_of_bags * num_of_bags))
            }
        }
    }
    count - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let input = input_generator(input);
        assert_eq!(part_1(&input), 4);
    }

    #[test]
    fn test_part2() {
        let input = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let input = input_generator(input);
        assert_eq!(part_2(&input), 126);
    }
}
