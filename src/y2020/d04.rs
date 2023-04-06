use std::collections::HashMap;

use crate::file_reader::*;

fn get_input(path: &str) -> String {
    let input = get_string_from_file_please(path);
    normalize_input(&input)
}

fn normalize_input(input: &str) -> String {
    input
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>().join(" "))
        .collect::<Vec<String>>()
        .join("\n")
}

fn line_to_map(line: &str) -> HashMap<&str, &str> {
    line.split(" ")
        .map(|s| s.split(":").collect::<Vec<&str>>())
        .map(|v| (v[0], v[1]))
        .collect()
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
}
