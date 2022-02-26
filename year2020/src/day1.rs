#[cfg(test)]
mod tests {
    fn solve_part_1(input: &str, output: u32) -> Option<u32> {
        let numbers: Vec<u32> = input.lines().map(|v| v.parse::<u32>().unwrap()).collect();

        for i in 0..numbers.len() {
            for j in (i + 1)..numbers.len() {
                if numbers[i] + numbers[j] == output {
                    return Some(numbers[i] * numbers[j]);
                }
            }
        }

        None
    }

    fn solve_part_2(input: &str, output: u32) -> Option<u32> {
        let numbers: Vec<u32> = input.lines().map(|v| v.parse::<u32>().unwrap()).collect();

        for i in 0..numbers.len() {
            for j in (i + 1)..numbers.len() {
                for k in (j + 1)..numbers.len() {
                    if numbers[i] + numbers[j] + numbers[k] == output {
                        return Some(numbers[i] * numbers[j] * numbers[k]);
                    }
                }
            }
        }

        None
    }

    #[test]
    fn should_solve_part_1_preview() {
        let input = include_str!("../input/day1_test.txt");
        assert_eq!(solve_part_1(input, 2020).unwrap(), 514579);
    }

    #[test]
    fn should_solve_part_1() {
        let input = include_str!("../input/day1.txt");
        assert_eq!(solve_part_1(input, 2020).unwrap(), 545379);
    }

    #[test]
    fn should_solve_part_2_preview() {
        let input = include_str!("../input/day1_test.txt");
        assert_eq!(solve_part_2(input, 2020).unwrap(), 241861950);
    }

    #[test]
    fn should_solve_part_2() {
        let input = include_str!("../input/day1.txt");
        assert_eq!(solve_part_2(input, 2020).unwrap(), 257778836);
    }
}
