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

fn solve_task_universal(
    numbers: Vec<usize>,
    numbers_to_sum: usize,
    compare_to_num: usize,
) -> usize {
    let mut indexes: Vec<usize> = (0..numbers_to_sum).map(|x| x).collect();
    let numbers_len = numbers.len();
    let max_values: Vec<usize> = indexes
        .iter()
        .enumerate()
        .map(|(i, _)| numbers_len - (numbers_to_sum - i))
        .collect();
    println!("numbers_len: {numbers_len}\nnumbers_to_sum: {numbers_to_sum}");
    println!("{indexes:?}");
    println!("{max_values:?}");
    loop {
        let to_sum: Vec<usize> = indexes.iter().map(|x| numbers[*x]).collect();
        if to_sum.iter().sum::<usize>() == compare_to_num {
            return to_sum.iter().fold(1, |acc, x| acc * x);
        }
        for ii in (0..numbers_to_sum).rev() {
            let i = indexes[ii];
            if i + 1 > max_values[ii] {
                if ii == 0 {
                    return 0;
                }
                continue;
            }
            let mut incr_by = 1;
            for iiu in ii..numbers_to_sum {
                indexes[iiu] = i + incr_by;
                incr_by = incr_by + 1;
            }
            break;
        }
    }
}

fn get_numbers_from_file(path: &str) -> Vec<usize> {
    let input = get_string_from_file_please(path);
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_task_universal_sample() {
        let numbers = get_numbers_from_file("ianda/2020/01s.txt");

        let expected1 = get_usize_from_file_please("ianda/2020/01sa1.txt");
        let expected2 = get_usize_from_file_please("ianda/2020/01sa2.txt");

        let s1 = solve_task_universal(numbers.clone(), 2, 2020);
        let s2 = solve_task_universal(numbers.clone(), 3, 2020);
        println!("s1: {s1}\ns2: {s2}");
        assert_eq!(s1, expected1);
        assert_eq!(s2, expected2);
    }

    #[test]
    fn test_solve_task_universal_input() {
        let numbers = get_numbers_from_file("ianda/2020/01i.txt");

        let expected1 = get_usize_from_file_please("ianda/2020/01a1.txt");
        let expected2 = get_usize_from_file_please("ianda/2020/01a2.txt");

        let s1 = solve_task_universal(numbers.clone(), 2, 2020);
        let s2 = solve_task_universal(numbers.clone(), 3, 2020);
        println!("s1: {s1}\ns2: {s2}");
        assert_eq!(s1, expected1);
        assert_eq!(s2, expected2);
    }

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
