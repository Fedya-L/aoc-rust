use crate::file_reader::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reader_module_access() {
        let value = get_usize_from_file_please("src/file_reader_test_usize.txt");
        assert_eq!(value, 54321);
    }
}
