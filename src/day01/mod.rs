const DOCUMENT: &str = include_str!("calibration_document.txt");
const CONVERSION_TABLE: [(&str, u8); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn main() {
    let sum = puzzle_one(DOCUMENT);
    println!("part one: {sum}");
    let sum = puzzle_two(DOCUMENT);
    println!("part two: {sum}");
}

fn puzzle_two(document: &str) -> u32 {
    parse_lines(document, get_line_sum_part_two)
}

fn puzzle_one(document: &str) -> u32 {
    parse_lines(document, get_line_sum_part_one)
}

fn parse_lines<F: Fn(&str) -> u32>(document: &str, sum_first_and_last: F) -> u32 {
    document.lines().map(sum_first_and_last).sum()
}

fn get_line_sum_part_one(line: &str) -> u32 {
    let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    *digits.first().unwrap() * 10 + *digits.last().unwrap()
}

fn get_line_sum_part_two(line: &str) -> u32 {
    let mut matches = Vec::new();
    for (number_str, val) in CONVERSION_TABLE {
        for (idx, _) in line.match_indices(number_str) {
            matches.push((idx, val));
        }
    }
    matches.sort_by_key(|(idx, _)| *idx);
    (matches.first().unwrap().1 * 10 + matches.last().unwrap().1) as u32
}

#[cfg(test)]
mod tests {
    use crate::day01::puzzle_two;

    use super::puzzle_one;

    #[test]
    fn example_puzzle_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let sum = puzzle_one(input);
        assert_eq!(sum, 142);
    }

    #[test]
    fn example_puzzle_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let sum = puzzle_two(input);
        assert_eq!(sum, 281);
    }
}
