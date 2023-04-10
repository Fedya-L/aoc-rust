use std::collections::{HashMap, HashSet};

use crate::file_reader::get_string_from_file_please;

fn solve_task_1(filepath: &str) -> usize {
    get_string_from_file_please(filepath)
        .split("\n\n")
        .map(|g| g.replace("\n", ""))
        .map(|g| HashSet::<char>::from_iter(g.chars()))
        .fold(0, |s, hs| s + hs.len())
}

fn solve_task_2(filepath: &str) -> usize {
    get_string_from_file_please(filepath)
        .trim_end()
        .split("\n\n")
        .map(|ga| count_group_answers(ga))
        .fold(0, |acc, count| acc + count)
}

fn count_group_answers(group_answers: &str) -> usize {
    let mut hs: HashMap<char, usize> = HashMap::new();
    let members_count = group_answers.lines().count();
    group_answers.replace(" ", "").chars().for_each(|c| {
        hs.insert(c, hs.get(&c).unwrap_or(&0) + 1);
    });
    hs.iter().filter(|t| *t.1 == members_count).count()
}

mod tests {
    use super::*;
    use crate::file_reader::get_usize_from_file_please;

    #[test]
    fn test_count_group_answers_1() {
        let input = "abc
    ab
    ac";
        let result = count_group_answers(input);
        assert_eq!(result, 1)
    }

    #[test]
    fn test_count_group_answers_2() {
        let input = "abc
    ab
    abce";
        let result = count_group_answers(input);
        assert_eq!(result, 2)
    }

    #[test]
    fn test_solve_task_2_sample() {
        let result = solve_task_2("ianda/2020/06/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/06/sa2.txt");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_2_input() {
        let result = solve_task_2("ianda/2020/06/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/06/ra2.txt");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_sample() {
        let result = solve_task_1("ianda/2020/06/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/06/sa1.txt");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_input() {
        let result = solve_task_1("ianda/2020/06/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/06/ra1.txt");

        assert_eq!(result, expected);
    }
}
