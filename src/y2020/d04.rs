use std::collections::HashMap;

use crate::file_reader::*;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn get_input(path: &str) -> String {
    let input = get_string_from_file_please(path);
    normalize_input(&input)
}

fn normalize_input(input: &str) -> String {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>().join(" "))
        .collect::<Vec<String>>()
        .join("\n")
}

fn line_to_map(line: &str) -> HashMap<&str, &str> {
    println!("line: '{}'", line);
    line.split(" ")
        .map(|s| s.split(":").collect::<Vec<&str>>())
        .map(|v| (v[0], v[1]))
        .collect()
}

fn validate_map(map: &HashMap<&str, &str>) -> bool {
    for field in REQUIRED_FIELDS.iter() {
        if !map.contains_key(field) {
            return false;
        }
    }
    true
}

fn solve_task_1(filepath: &str) -> usize {
    get_input(filepath)
        .lines()
        .map(|l| line_to_map(l))
        .filter(|m| validate_map(m))
        .count()
}

mod tests {
    use super::*;

    #[test]
    fn test_normalize_input() {
        let input = "line1
line2

line3

line4
line5";
        let expected = "line1 line2
line3
line4 line5";

        let result = normalize_input(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_line_to_map() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd";
        let expected = vec![
            ("ecl", "gry"),
            ("pid", "860033327"),
            ("eyr", "2020"),
            ("hcl", "#fffffd"),
        ];

        let result = line_to_map(input);

        for (k, v) in expected {
            assert_eq!(result.get(k), Some(&v));
        }
    }

    #[test]
    fn test_validate_map_false() {
        let input = vec![
            ("ecl", "gry"),
            ("pid", "860033327"),
            ("eyr", "2020"),
            ("hcl", "#fffffd"),
        ];

        let map: HashMap<&str, &str> = input.into_iter().collect();

        let result = validate_map(&map);

        assert_eq!(result, false);
    }

    #[test]
    fn test_validate_map_true() {
        let input = vec![
            ("ecl", "gry"),
            ("eyr", "2020"),
            ("hcl", "#fffffd"),
            ("byr", "1937"),
            ("iyr", "2017"),
            ("pid", "147"),
            ("hgt", "183cm"),
        ];

        let map: HashMap<&str, &str> = input.into_iter().collect();

        let result = validate_map(&map);

        assert_eq!(result, true);
    }

    #[test]
    fn test_solve_task_1_sample() {
        let result = solve_task_1("ianda/2020/04/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/04/sa1.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_input() {
        let result = solve_task_1("ianda/2020/04/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/04/ra1.txt");
        assert_eq!(result, expected);
    }

    /*
    #[test]
    fn test_solve_task_2_sample() {
        let result = solve_task_2("ianda/2020/04/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/04/sa2.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_2_input() {
        let result = solve_task_2("ianda/2020/04/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/04/ra2.txt");
        assert_eq!(result, expected);
    }
    */
}
