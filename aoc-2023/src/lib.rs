pub fn solve_part_1(input: &str) -> u32 {
    let mut calibration_value = 0;
    for line in input.lines() {
        let first_digit = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();

        calibration_value += first_digit * 10 + last_digit;
    }

    calibration_value
}

const DIGITS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn digit_to_value(digit: &str) -> u32 {
    match digit {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("Unknown digit: {}", digit),
    }
}

fn starts_with(input: &str, with: &str) -> bool {
    if with.len() > input.len() {
        return false;
    }
    &input[0..with.len()] == with
}

pub fn solve_part_2(input: &str) -> u32 {
    let mut calibration_value = 0;
    for line in input.lines() {
        let mut first_value: Option<u32> = None;

        'outer: for i in 0..line.len() {
            let substring = &line[i..];

            for digit in DIGITS {
                if starts_with(substring, digit) {
                    first_value = Some(digit_to_value(digit));
                    break 'outer;
                }
            }
        }

        let mut last_value: Option<u32> = None;
        for i in 0..line.len() {
            let substring = &line[i..];

            for digit in DIGITS {
                if starts_with(substring, digit) {
                    last_value = Some(digit_to_value(digit));
                }
            }
        }

        let first_value = first_value.unwrap();
        let last_value = last_value.unwrap();

        calibration_value += first_value * 10 + last_value;
    }

    calibration_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1_example() {
        let input = include_str!("../input/day_1_part_1_example.txt");

        let result = solve_part_1(input);

        assert_eq!(result, 142);
    }

    #[test]
    fn solves_part_1_puzzle() {
        let input = include_str!("../input/day_1_puzzle.txt");

        let result = solve_part_1(input);

        assert_eq!(result, 55002);
    }

    #[test]
    fn solves_part_2_example() {
        let input = include_str!("../input/day_1_part_2_example.txt");

        let result = solve_part_2(input);

        assert_eq!(result, 281);
    }

    #[test]
    fn solves_part_2_puzzle() {
        let input = include_str!("../input/day_1_puzzle.txt");

        let result = solve_part_2(input);

        assert_eq!(result, 281);
    }
}
