use crate::file_reader::*;
use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

struct Pattern {
    tree_coordinates: HashSet<[usize; 2]>,
    width: usize,
    height: usize,
}

impl FromStr for Pattern {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tree_coordinates: HashSet<[usize; 2]> = HashSet::new();
        let mut y = 0;
        let mut x = 0;
        for line in s.lines() {
            x = 0;
            for char in line.chars() {
                if char == '#' {
                    tree_coordinates.insert([x, y]);
                }
                x = x + 1;
            }
            y = y + 1;
        }
        Ok(Pattern {
            tree_coordinates,
            width: x,
            height: y,
        })
    }
}

fn solve_task_1(filename: &str) -> usize {
    let input = get_string_from_file_please(filename);
    let pattern = input.parse::<Pattern>().unwrap();

    let (mut x, mut y) = (0, 0);

    calculate_trees_on_slope(&pattern, [3, 1])
}

fn calculate_trees_on_slope(pattern: &Pattern, slope: [usize; 2]) -> usize {
    let (mut x, mut y) = (0, 0);

    let mut tree_count = 0;

    while y < pattern.height {
        if pattern.tree_coordinates.contains(&[x % pattern.width, y]) {
            tree_count = tree_count + 1;
        }
        x = x + slope[0];
        y = y + slope[1];
    }

    tree_count
}

fn solve_task_2(filename: &str) -> usize {
    let input = get_string_from_file_please(filename);
    let pattern = input.parse::<Pattern>().unwrap();
    let slopes: [[usize; 2]; 5] = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    slopes
        .map(|slope| calculate_trees_on_slope(&pattern, slope))
        .iter()
        .fold(1, |a, v| a * v)
}

mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = get_string_from_file_please("ianda/2020/03/si.txt");

        let pattern = input.parse::<Pattern>().unwrap();

        assert_eq!(pattern.tree_coordinates.len(), 37);
        assert_eq!(pattern.width, 11);
        assert_eq!(pattern.height, 11);
    }

    #[test]
    fn test_hash_set() {
        let mut hs: HashSet<[usize; 2]> = HashSet::new();
        hs.insert([0, 0]);
        hs.insert([1, 1]);

        for i in 0..2 {
            for ii in 0..2 {
                let c = [i, ii];
                println!("Does {:?} exist? {}", c, hs.contains(&c));
                assert_eq!(hs.contains(&c), c[0] == c[1]);
            }
        }
    }

    #[test]
    fn test_solve_task_1_sample() {
        let result = solve_task_1("ianda/2020/03/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/03/sa1.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_input() {
        let result = solve_task_1("ianda/2020/03/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/03/ra1.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_2_sample() {
        let result = solve_task_2("ianda/2020/03/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/03/sa2.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_2_input() {
        let result = solve_task_2("ianda/2020/03/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/03/ra2.txt");
        assert_eq!(result, expected);
    }
}
