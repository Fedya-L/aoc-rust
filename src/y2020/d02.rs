use crate::file_reader::*;
use std::str::FromStr;

struct PolicyAndPassword {
    required_letter: char,
    min_count: usize,
    max_count: usize,
    the_password: String,
}

#[derive(Debug)]
struct ParsingError;

impl FromStr for PolicyAndPassword {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        let range_parts: Vec<&str> = parts[0].split("-").collect();
        let min_count = range_parts[0].parse::<usize>().unwrap();
        let max_count = range_parts[1].parse::<usize>().unwrap();

        let required_letter = parts[1].chars().next().unwrap();

        let the_password = parts[2];

        return Ok(PolicyAndPassword {
            min_count,
            max_count,
            required_letter,
            the_password: the_password.to_string(),
        });
    }
}

impl PolicyAndPassword {
    fn is_valid(&self) -> bool {
        let mut count = 0;
        self.the_password.chars().for_each(|c| {
            if c == self.required_letter {
                count = count + 1
            }
        });
        count >= self.min_count && count <= self.max_count
    }
}

fn solve_task_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.parse::<PolicyAndPassword>().unwrap())
        .filter(|p| p.is_valid())
        .count()
}

mod tests {
    use super::*;

    #[test]
    fn test_pap_is_valid() {
        let cases: Vec<(&str, bool)> = vec![("1-3 a: abcde", true), ("2-3 a: abcde", false)];

        for case in cases {
            let pop = PolicyAndPassword::from_str(case.0).unwrap();
            assert_eq!(pop.is_valid(), case.1);
        }
    }

    #[test]
    fn test_pap_from_string() {
        let string = "1-3 a: abcde";

        let pap = PolicyAndPassword::from_str(string).unwrap();

        assert_eq!(pap.required_letter, 'a');
        assert_eq!(pap.min_count, 1);
        assert_eq!(pap.max_count, 3);
        assert_eq!(pap.the_password, "abcde");
    }

    #[test]
    fn test_sample_answer_1() {
        let expected = get_usize_from_file_please("ianda/2020/02/sa1.txt");

        let input = get_string_from_file_please("ianda/2020/02/si.txt");
        let result = solve_task_1(&input);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_real_answer_1() {
        let expected = get_usize_from_file_please("ianda/2020/02/ra1.txt");

        let input = get_string_from_file_please("ianda/2020/02/ri.txt");
        let result = solve_task_1(&input);

        assert_eq!(result, expected)
    }
}
