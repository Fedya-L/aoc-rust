use std::{collections::HashSet, str::FromStr};

use crate::file_reader::get_string_from_file_please;

enum Instruction {
    Nop,
    Acc(isize),
    Jmp(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let instruction = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<isize>().unwrap();

        match instruction {
            "nop" => Ok(Instruction::Nop),
            "acc" => Ok(Instruction::Acc(value)),
            "jmp" => Ok(Instruction::Jmp(value)),
            _ => Err(()),
        }
    }
}

fn get_instructions(filepath: &str) -> Vec<Instruction> {
    get_string_from_file_please(filepath)
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect()
}

fn solve_task_1(filepath: &str) -> isize {
    let instructions = get_instructions(filepath);
    let mut visited = HashSet::<usize>::new();

    let mut register = 0;
    let mut index = 0;

    while !visited.contains(&index) {
        visited.insert(index);

        match instructions[index] {
            Instruction::Nop => index += 1,
            Instruction::Acc(value) => {
                register += value;
                index += 1;
            }
            Instruction::Jmp(value) => index = ((index as isize) + value) as usize,
        }
    }

    register
}

fn solve_task_2(filepath: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::file_reader::{get_isize_from_file_please, get_usize_from_file_please};

    use super::*;

    #[test]
    fn test_solve_task_2_sample() {
        let result = solve_task_2("ianda/2020/08/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/08/sa2.txt");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_2_input() {
        let result = solve_task_2("ianda/2020/08/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/08/ra2.txt");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_sample() {
        let result = solve_task_1("ianda/2020/08/si.txt");
        let expected = get_isize_from_file_please("ianda/2020/08/sa1.txt");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_input() {
        let result = solve_task_1("ianda/2020/08/ri.txt");
        let expected = get_isize_from_file_please("ianda/2020/08/ra1.txt");

        assert_eq!(result, expected);
    }
}
