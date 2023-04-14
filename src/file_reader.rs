use std::fs::File;
use std::io::prelude::*;

pub fn get_string_from_file_please(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
}

pub fn get_usize_from_file_please(path: &str) -> usize {
    let string = get_string_from_file_please(path);
    string.trim().parse().expect("Not a number")
}

pub fn get_isize_from_file_please(path: &str) -> isize {
    let string = get_string_from_file_please(path);
    string.trim().parse().expect("Not a number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_string_from_file_please("src/file_reader.rs");
        let first_line = result.lines().next().unwrap();
        assert_eq!(first_line, "use std::fs::File;");
    }

    #[test]
    fn test_usize() {
        let result = get_usize_from_file_please("src/file_reader_test_usize.txt");
        assert_eq!(result, 54321);
    }

    #[test]
    fn test_isize() {
        let result = get_isize_from_file_please("src/file_reader_test_isize.txt");
        let expected = -4321;
        assert_eq!(result, expected);
    }
}
