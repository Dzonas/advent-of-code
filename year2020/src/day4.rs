#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::collections::{HashMap, HashSet};

    lazy_static! {
        static ref REQUIRED_FIELDS: HashSet<&'static str> =
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .cloned()
                .collect();
        static ref HCL_POSSIBLE_LETTERS: HashSet<char> =
            ['a', 'b', 'c', 'd', 'e', 'f'].iter().cloned().collect();
        static ref ECL_POSSIBLE_VALUES: HashSet<&'static str> =
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .cloned()
                .collect();
    }

    fn passport_to_properties(passport: &str) -> HashMap<&str, &str> {
        passport
            .split(char::is_whitespace)
            .filter(|&field| field != "")
            .map(|field| field.split(':').collect::<Vec<&str>>())
            .map(|field_pair| (field_pair[0], field_pair[1]))
            .collect::<HashMap<&str, &str>>()
    }

    fn has_all_required_fields(passport_fields: &HashSet<&str>) -> bool {
        REQUIRED_FIELDS.is_subset(&passport_fields)
    }

    fn is_in_range(n: usize, min: usize, max: usize) -> bool {
        n >= min && n <= max
    }

    fn is_byr_valid(byr: &str) -> bool {
        let byr_number: usize = byr.parse().unwrap();
        is_in_range(byr_number, 1920, 2002)
    }

    fn is_iyr_valid(iyr: &str) -> bool {
        let iyr_number: usize = iyr.parse().unwrap();
        is_in_range(iyr_number, 2010, 2020)
    }

    fn is_eyr_valid(eyr: &str) -> bool {
        let eyr_number: usize = eyr.parse().unwrap();
        is_in_range(eyr_number, 2020, 2030)
    }

    fn is_hgt_valid(hgt: &str) -> bool {
        let hgt_number: usize = hgt
            .chars()
            .filter(|&c| char::is_numeric(c))
            .collect::<String>()
            .parse()
            .unwrap();
        let unit = hgt
            .chars()
            .filter(|&c| char::is_alphabetic(c))
            .collect::<String>();
        let min;
        let max;

        if unit == "cm" {
            min = 150;
            max = 193;
        } else if unit == "in" {
            min = 59;
            max = 76;
        } else {
            return false;
        }

        is_in_range(hgt_number, min, max)
    }

    fn is_hcl_valid(hcl: &str) -> bool {
        let mut hcl_iter = hcl.chars();

        if hcl_iter.next().unwrap() != '#' {
            return false;
        }

        hcl_iter
            .filter(|c| char::is_numeric(*c) || HCL_POSSIBLE_LETTERS.contains(c))
            .count()
            == 6
    }

    fn is_ecl_valid(ecl: &str) -> bool {
        ECL_POSSIBLE_VALUES.contains(&ecl)
    }

    fn is_pid_valid(pid: &str) -> bool {
        pid.chars().filter(|&c| char::is_numeric(c)).count() == 9
    }

    fn are_all_fields_valid(passport_properties: &HashMap<&str, &str>) -> bool {
        let validators: Vec<(&str, &dyn Fn(&str) -> bool)> = vec![
            ("byr", &is_byr_valid),
            ("iyr", &is_iyr_valid),
            ("eyr", &is_eyr_valid),
            ("hgt", &is_hgt_valid),
            ("hcl", &is_hcl_valid),
            ("ecl", &is_ecl_valid),
            ("pid", &is_pid_valid),
        ];

        validators
            .iter()
            .all(|&(field, fun)| fun(passport_properties[field]))
    }

    fn is_passport_valid(passport_properties: &HashMap<&str, &str>) -> bool {
        has_all_required_fields(&passport_properties.keys().cloned().collect())
            && are_all_fields_valid(passport_properties)
    }

    fn solve_part_1(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|passport| passport_to_properties(passport))
            .filter(|passport| {
                has_all_required_fields(&passport.keys().cloned().collect::<HashSet<&str>>())
            })
            .count()
    }

    fn solve_part_2(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|passport| passport_to_properties(passport))
            .filter(|properties| is_passport_valid(properties))
            .count()
    }

    #[test]
    fn should_solve_part_1_preview() {
        let input = include_str!("../input/day4_test_part_1.txt");
        assert_eq!(solve_part_1(input), 2);
    }

    #[test]
    fn should_solve_part_1() {
        let input = include_str!("../input/day4.txt");
        assert_eq!(solve_part_1(input), 247);
    }

    #[test]
    fn should_solve_part_2_preview() {
        let input = include_str!("../input/day4_test_part_2.txt");
        assert_eq!(solve_part_2(input), 4);
    }

    #[test]
    fn should_solve_part_2() {
        let input = include_str!("../input/day4.txt");
        assert_eq!(solve_part_2(input), 145);
    }
}
