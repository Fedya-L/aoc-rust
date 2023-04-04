use crate::file_reader::*;
use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

struct Pattern {
    tree_coordinates: HashSet<[usize; 2]>,
}

impl FromStr for Pattern {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tree_coordinates: HashSet<[usize; 2]> = HashSet::new();
        let mut y = 0;
        for line in s.lines() {
            let mut x = 0;
            for char in line.chars() {
                if char == '#' {
                    tree_coordinates.insert([x, y]);
                }
                x = x + 1;
            }
            y = y + 1;
        }
        Ok(Pattern { tree_coordinates })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = get_string_from_file_please("ianda/2020/03/si.txt");

        let pattern = input.parse::<Pattern>().unwrap();

        assert_eq!(pattern.tree_coordinates.len(), 37);
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
}
