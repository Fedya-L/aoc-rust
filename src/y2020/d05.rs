use crate::file_reader::get_string_from_file_please;

fn string_to_two_numbers(str: &str) -> (usize, usize) {
    let binary_numbers: String = str
        .chars()
        .map(|c| match c {
            'F' => '0',
            'B' => '1',
            'L' => '0',
            'R' => '1',
            _ => panic!("This should not happen..."),
        })
        .collect();
    let (b1, b2) = binary_numbers.split_at(7);
    (
        usize::from_str_radix(b1, 2).unwrap(),
        usize::from_str_radix(b2, 2).unwrap(),
    )
}

fn solve_task_1(filepath: &str) -> usize {
    let input = get_string_from_file_please(filepath);
    input
        .lines()
        .map(|l| string_to_two_numbers(l))
        .map(|ns| ns.0 * 8 + ns.1)
        .max()
        .unwrap()
}

mod d05_tests {
    use crate::file_reader::get_usize_from_file_please;

    use super::*;

    #[test]
    fn test_string_to_two_numbers_min() {
        let result = string_to_two_numbers("FFFFFFFLLL");
        let expected = (0usize, 0usize);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_to_two_numbers_max() {
        let result = string_to_two_numbers("BBBBBBBRRR");
        let expected = (127usize, 7usize);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_sample() {
        let result = solve_task_1("ianda/2020/05/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/05/sa1.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_input() {
        let result = solve_task_1("ianda/2020/05/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/05/ra1.txt");
        assert_eq!(result, expected);
    }
}
