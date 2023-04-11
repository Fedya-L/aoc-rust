struct Option(String, usize);

struct Rule {
    color: String,
    options: Vec<Option>,
}

fn line_to_rule(line: &str) -> Rule {
    let split = line.split(" contain ");

    Rule {
        color: "CoLoUR".to_string(),
        options: Vec::new(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_line_to_rule() {
        let line = "mirrored tomato bags contain 3 light yellow bags, 4 dim indigo bags, 5 light beige bags, 3 posh indigo bags.";
    }

    #[test]
    fn test_split_last() {
        let input = "dim orange bags";
        let expected = "dim orange";
        let result = input.split(" bags").next().unwrap_or("default");

        assert_eq!(result, expected)
    }
}
