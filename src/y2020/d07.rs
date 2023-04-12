struct Option(String, usize);

struct Rule {
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
        .collect::<Vec<Option>>();

    Rule {
        color: color.to_string(),
        options: options,
    }
}

#[cfg(test)]
mod tests {
    use super::line_to_rule;

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
