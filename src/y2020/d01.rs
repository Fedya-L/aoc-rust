use crate::file_reader::*;

fn solve_sample1() -> usize {
    let input = get_string_from_file_please("ianda/2020/01s.txt");
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
        let result = solve_sample1();
        let expected = get_usize_from_file_please("ianda/2020/01sa1.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_file_reader_module_access() {
        let value = get_usize_from_file_please("src/file_reader_test_usize.txt");
        assert_eq!(value, 54321);
    }
}
