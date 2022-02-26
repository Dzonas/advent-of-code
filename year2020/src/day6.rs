#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    fn solve_part_1(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|group| {
                group
                    .replace("\n", "")
                    .chars()
                    .collect::<HashSet<char>>()
                    .len()
            })
            .sum()
    }

    fn get_yes_count(people_answers: &[&str]) -> usize {
        let people_count = people_answers.len();
        let mut yes_count = HashMap::new();

        people_answers
            .join("")
            .replace("\n", "")
            .chars()
            .for_each(|c| {
                yes_count.entry(c).and_modify(|e| *e += 1).or_insert(1);
            });

        yes_count.values().filter(|v| **v == people_count).count()
    }

    fn solve_part_2(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|group_answers| group_answers.lines().collect::<Vec<&str>>())
            .map(|people_answers| get_yes_count(&people_answers))
            .sum()
    }

    #[test]
    fn test_should_solve_part_1_preview() {
        let input = include_str!("../input/day6_test.txt");
        assert_eq!(solve_part_1(input), 11);
    }

    #[test]
    fn test_should_solve_part_1() {
        let input = include_str!("../input/day6.txt");
        assert_eq!(solve_part_1(input), 6443);
    }

    #[test]
    fn test_should_solve_part_2_preview() {
        let input = include_str!("../input/day6_test.txt");
        assert_eq!(solve_part_2(input), 6);
    }

    #[test]
    fn test_should_solve_part_2() {
        let input = include_str!("../input/day6.txt");
        assert_eq!(solve_part_2(input), 3232);
    }
}
