use std::collections::HashMap;

struct Input {
    x: Vec<u32>,
    y: Vec<u32>,
}

fn main() {
    let input = include_str!("../../inputs/1.txt");
    let parsed_input = parse(input);

    println!("{}", solve_part_1(&parsed_input));
    println!("{}", solve_part_2(&parsed_input));
}

fn parse(input: &str) -> Input {
    let mut x = Vec::new();
    let mut y = Vec::new();

    for line in input.lines() {
        let mut splitted_line = line.split_whitespace();
        let a: u32 = splitted_line
            .next()
            .expect("first column should not be empty")
            .parse()
            .expect("value in the first column should be a positive integer");
        let b: u32 = splitted_line
            .next()
            .expect("second column should not be empty")
            .parse()
            .expect("value in the second columnd should be a positive integer");

        x.push(a);
        y.push(b);
    }

    x.sort();
    y.sort();

    Input { x, y }
}

fn solve_part_1(input: &Input) -> u32 {
    input
        .x
        .iter()
        .zip(&input.y)
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn solve_part_2(input: &Input) -> u32 {
    let mut map = HashMap::new();
    for b in &input.y {
        *map.entry(*b).or_insert(0) += 1;
    }

    input.x.iter().map(|v| *v * map.get(v).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "3   4\n\
		     4   3\n\
		     2   5\n\
		     1   3\n\
		     3   9\n\
		     3   3";

    #[test]
    fn test_part_1() {
        let result = solve_part_1(&parse(INPUT));

        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2(&parse(INPUT));

        assert_eq!(result, 31);
    }
}
