use std::collections::HashSet;

use crate::file_reader::get_string_from_file_please;

#[derive(PartialEq, Eq, Debug, Hash)]
struct Seat(usize, usize);

fn string_to_seat(str: &str) -> Seat {
    let binary_numbers: String = str
        .chars()
        .map(|c| match c {
            'F' => '0',
            'B' => '1',
            'L' => '0',
            'R' => '1',
            _ => panic!("This should not happen..."),
        })
        .collect();
    let (b1, b2) = binary_numbers.split_at(7);
    Seat(
        usize::from_str_radix(b1, 2).unwrap(),
        usize::from_str_radix(b2, 2).unwrap(),
    )
}

fn get_id_from_seat(seat: &Seat) -> usize {
    seat.0 * 8 + seat.1
}

fn solve_task_1(filepath: &str) -> usize {
    let input = get_string_from_file_please(filepath);
    input
        .lines()
        .map(|l| string_to_seat(l))
        .map(|ns| get_id_from_seat(&ns))
        .max()
        .unwrap()
}

fn solve_task_2(filepath: &str) -> usize {
    let input = get_string_from_file_please(filepath);
    let seats: HashSet<Seat> = HashSet::from_iter(input.lines().map(|l| string_to_seat(l)));

    let mut saw_full_row = false;
    for row in 0..128 {
        for col in 0..8 {
            if seats.contains(&Seat(row, col)) {
                print!("#");
                if saw_full_row == false && col == 7 {
                    saw_full_row = true;
                }
            } else {
                print!("x");
                if saw_full_row {
                    return get_id_from_seat(&Seat(row, col));
                } else {
                    break;
                }
            }
        }
        print!("\n");
    }
    0
}
mod tests {
    use crate::file_reader::get_usize_from_file_please;

    use super::*;

    #[test]
    fn test_solve_task_2_input() {
        let result = solve_task_2("ianda/2020/05/ri.txt");

        let expected = get_usize_from_file_please("ianda/2020/05/ra1.txt");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_to_two_numbers_min() {
        let result = string_to_seat("FFFFFFFLLL");
        let expected = Seat(0usize, 0usize);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_string_to_two_numbers_max() {
        let result = string_to_seat("BBBBBBBRRR");
        let expected = Seat(127usize, 7usize);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_sample() {
        let result = solve_task_1("ianda/2020/05/si.txt");
        let expected = get_usize_from_file_please("ianda/2020/05/sa1.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_task_1_input() {
        let result = solve_task_1("ianda/2020/05/ri.txt");
        let expected = get_usize_from_file_please("ianda/2020/05/ra1.txt");
        assert_eq!(result, expected);
    }
}
