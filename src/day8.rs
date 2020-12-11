use std::collections::HashSet;
#[derive(Debug, Clone)]
pub enum OpCode {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

pub enum Status {
    Loop(i32),
    End(i32),
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<OpCode> {
    input
        .lines()
        .map(|c| {
            let parts: Vec<_> = c.split(" ").collect();
            let step: i32 = parts[1].parse().unwrap();
            match parts[0] {
                "acc" => OpCode::Acc(step),
                "jmp" => OpCode::Jmp(step),
                "nop" => OpCode::Nop(step),
                _ => panic!("Failed to parse opcode"),
            }
        })
        .collect()
}

fn run_program(program: &[OpCode]) -> Status {
    let mut cursor: usize = 0;
    let mut accumulator = 0;
    let mut seen = HashSet::new();

    loop {
        if !seen.insert(cursor) {
            return Status::Loop(accumulator);
        }
        if cursor >= program.len() {
            return Status::End(accumulator);
        }

        let code = &program[cursor];
        println!("{:?} {}", code, accumulator);
        match code {
            OpCode::Acc(s) => {
                accumulator += s;
                cursor += 1;
            }
            OpCode::Jmp(s) => cursor = (cursor as i32 + s) as usize,
            OpCode::Nop(_) => cursor += 1,
        }
    }
}

#[aoc(day8, part1)]
pub fn part_1(input: &[OpCode]) -> i32 {
    match run_program(input) {
        Status::End(_) => 0,
        Status::Loop(acc) => acc,
    }
}

#[aoc(day8, part2)]
pub fn part_2(input: &[OpCode]) -> i32 {
    for (index, code) in input.iter().enumerate() {
        let mut input_copy = input.to_vec();
        let entry = input_copy.get_mut(index).unwrap();
        match code {
            OpCode::Acc(_) => continue,
            &OpCode::Jmp(s) => *entry = OpCode::Nop(s),
            &OpCode::Nop(s) => *entry = OpCode::Jmp(s),
        };
        println!("{:?}", input_copy);
        if let Status::End(accumulator) = run_program(&input_copy) {
            return accumulator;
        };
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let input = input_generator(input);
        assert_eq!(part_1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let input = input_generator(input);
        assert_eq!(part_2(&input), 8);
    }
}
