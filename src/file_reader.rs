use std::fs::File;
use std::io::prelude::*;

fn get_string_from_file_please(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    contents
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
}
