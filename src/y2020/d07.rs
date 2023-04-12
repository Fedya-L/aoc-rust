use std::collections::HashMap;

use crate::file_reader::get_string_from_file_please;

// Will use normal stucts with named fields next time.
struct Option(String, usize);

struct Rule {
    // Tried to use &str, but it didn't work.
    color: String,
    options: Vec<Option>,
}

fn line_to_rule(line: &str) -> Rule {
    let (color, raw_options) = line.split_once(" bags contain ").unwrap();

    let raw_options = raw_options.split(", ");
    let options = raw_options
        .map(|o| {
            let mut split = o.split_ascii_whitespace();
            let count = split.next().unwrap_or("0").parse::<usize>().unwrap_or(0);
            _ = split.next_back();
            let color = split.collect::<Vec<&str>>().join(" ");
            Option(color, count)
        })
        // "no other bags" text screwed me over.
        .filter(|r| r.1 > 0)
        .collect::<Vec<Option>>();

    Rule {
        color: color.to_string(),
        options: options,
    }
}

fn solve_task_1(filepath: &str) -> usize {
    let hm: HashMap<String, Rule> = get_string_from_file_please(filepath)
        .lines()
        .map(|l| line_to_rule(l))
        .map(|r| (r.color.clone(), r))
        .collect();

    hm.iter()
        .filter(|t| can_contain(&hm, t.0, "shiny gold"))
        .count()
}

fn can_contain(hm: &HashMap<String, Rule>, color: &str, target: &str) -> bool {
    let rule = hm.get(color).unwrap();
    rule.options
        .iter()
        .any(|o| o.0 == target || can_contain(hm, o.0.as_str(), target))
}

#[cfg(test)]
mod tests {
    use crate::file_reader::get_usize_from_file_please;

    use super::*;

    #[test]
    fn test_solve_task_1_sample() {
        let result = solve_task_1("ianda/2020/07/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/07/sa1.txt");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_input() {
        let result = solve_task_1("ianda/2020/07/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/07/ra1.txt");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_line_to_rule() {
        let line = "mirrored tomato bags contain 3 light yellow bags, 4 dim indigo bags, 5 light beige bags, 3 posh indigo bags.";

        let rule = line_to_rule(line);

        assert_eq!(rule.color, "mirrored tomato");
        assert_eq!(rule.options[0].0, "light yellow");
        assert_eq!(rule.options[0].1, 3);
        assert_eq!(rule.options[1].0, "dim indigo");
        assert_eq!(rule.options[1].1, 4);
        assert_eq!(rule.options[2].0, "light beige");
        assert_eq!(rule.options[2].1, 5);
        assert_eq!(rule.options[3].0, "posh indigo");
        assert_eq!(rule.options[3].1, 3);
    }

    #[test]
    fn test_split_last() {
        let input = "dim orange bags";
        let expected = "dim orange";
        let result = input.split(" bags").next().unwrap_or("default");

        assert_eq!(result, expected)
    }
}
