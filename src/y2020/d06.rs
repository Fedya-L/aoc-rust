use std::collections::HashSet;

use crate::file_reader::get_string_from_file_please;

fn solve_task_1(filepath: &str) -> usize {
    get_string_from_file_please(filepath)
        .split("\n\n")
        .map(|g| g.replace("\n", ""))
        .map(|g| HashSet::<char>::from_iter(g.chars()))
        .fold(0, |s, hs| s + hs.len())
}

mod tests {
    use super::*;
    use crate::file_reader::get_usize_from_file_please;

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
