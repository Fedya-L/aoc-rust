use crate::file_reader::*;

fn solve_task1(path: &str) -> usize {
    let input = get_string_from_file_please(path);
    let numbers: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return numbers[i] * numbers[j];
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_sample1() {
        let result = solve_task1("ianda/2020/01s.txt");
        let expected = get_usize_from_file_please("ianda/2020/01sa1.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task1() {
        let result = solve_task1("ianda/2020/01i.txt");
        let expected = get_usize_from_file_please("ianda/2020/01a1.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_file_reader_module_access() {
        let value = get_usize_from_file_please("src/file_reader_test_usize.txt");
        assert_eq!(value, 54321);
    }
}
