#[cfg(test)]
mod tests {
    use regex::Regex;

    fn part_1_is_password_valid(
        min_count: usize,
        max_count: usize,
        c: char,
        password: &str,
    ) -> bool {
        let matched_character_count = password.chars().filter(|v| *v == c).count();
        matched_character_count >= min_count && matched_character_count <= max_count
    }

    fn part_2_is_password_valid(
        first_pos: usize,
        second_pos: usize,
        c: char,
        password: &str,
    ) -> bool {
        password
            .chars()
            .enumerate()
            .filter(|&(i, v)| (i + 1 == first_pos || i + 1 == second_pos) && v == c)
            .count()
            == 1
    }

    fn solve_core(input: &str, password_validator: fn(usize, usize, char, &str) -> bool) -> usize {
        let re =
            Regex::new(r"(?P<min_count>\d+)-(?P<max_count>\d+) (?P<char>\w): (?P<password>\w+)")
                .expect("unable to compile regex");
        re.captures_iter(input)
            .map(|caps| {
                (
                    caps["min_count"].parse::<usize>().unwrap(),
                    caps["max_count"].parse::<usize>().unwrap(),
                    caps["char"].parse::<char>().unwrap(),
                    caps.name("password").unwrap().as_str(),
                )
            })
            .filter(|&(min_count, max_count, c, password)| {
                password_validator(min_count, max_count, c, password)
            })
            .count()
    }

    fn solve_part_1(input: &str) -> usize {
        solve_core(input, part_1_is_password_valid)
    }

    fn solve_part_2(input: &str) -> usize {
        solve_core(input, part_2_is_password_valid)
    }

    #[test]
    fn should_solve_part_1_preview() {
        let input = include_str!("../input/day2_test.txt");
        assert_eq!(solve_part_1(input), 2);
    }

    #[test]
    fn should_solve_part_1() {
        let input = include_str!("../input/day2.txt");
        assert_eq!(solve_part_1(input), 564);
    }

    #[test]
    fn should_solve_part_2_preview() {
        let input = include_str!("../input/day2_test.txt");
        assert_eq!(solve_part_2(input), 1);
    }

    #[test]
    fn should_solve_part_2() {
        let input = include_str!("../input/day2.txt");
        assert_eq!(solve_part_2(input), 325);
    }
}
