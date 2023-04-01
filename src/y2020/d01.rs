use crate::file_reader::*;

fn solve_task1(path: &str) -> usize {
    let numbers = get_numbers_from_file(path);
    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return numbers[i] * numbers[j];
            }
        }
    }
    0
}

fn solve_task2(path: &str) -> usize {
    let numbers = get_numbers_from_file(path);
    for i in 0..numbers.len() - 2 {
        for j in i + 1..numbers.len() - 1 {
            for k in j + 1..numbers.len() {
                let i = numbers[i];
                let j = numbers[j];
                let k = numbers[k];
                if (i + j + k) == 2020 {
                    return i * k * j;
                }
            }
        }
    }
    0
}

fn get_numbers_from_file(path: &str) -> Vec<usize> {
    let input = get_string_from_file_please(path);
    input.lines().map(|x| x.parse().unwrap()).collect()
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
    fn test_solve_sample2() {
        let result = solve_task2("ianda/2020/01s.txt");
        let expected = get_usize_from_file_please("ianda/2020/01sa2.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task2() {
        let result = solve_task2("ianda/2020/01i.txt");
        let expected = get_usize_from_file_please("ianda/2020/01a2.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_file_reader_module_access() {
        let value = get_usize_from_file_please("src/file_reader_test_usize.txt");
        assert_eq!(value, 54321);
    }
}
