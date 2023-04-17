use nom::{
    character::complete::{alpha1, newline},
    combinator::verify,
    multi::many1,
    IResult,
};
use std::collections::HashSet;

type Item = char;

#[derive(Debug)]
struct Rucksack(Vec<Item>);

impl Rucksack {
    fn compartment_size(&self) -> usize {
        self.0.len() / 2
    }

    fn compartment1(&self) -> impl Iterator<Item = &Item> {
        self.0.iter().take(self.compartment_size())
    }

    fn compartment2(&self) -> impl Iterator<Item = &Item> {
        self.0.iter().skip(self.compartment_size())
    }

    fn shared_item(&self) -> char {
        let c1 = HashSet::<_>::from_iter(self.compartment1());
        let c2 = HashSet::<_>::from_iter(self.compartment2());

        **c1.intersection(&c2).next().unwrap()
    }
}

fn priority(c: char) -> u32 {
    let v = c as u8;

    if (65..=90).contains(&v) {
        (v - 38) as u32
    } else if (97..=122).contains(&v) {
        (v - 96) as u32
    } else {
        panic!("Char is not an ASCII character")
    }
}

fn parse_line(input: &str) -> IResult<&str, Rucksack> {
    let (input, output) = verify(alpha1, |s: &str| s.len() % 2 == 0)(input)?;
    let (input, _) = newline(input)?;

    let items = output.chars().collect();
    let rucksack = Rucksack(items);

    Ok((input, rucksack))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Rucksack>> {
    verify(many1(parse_line), |r: &Vec<Rucksack>| r.len() % 3 == 0)(input)
}

pub fn solve_part_1(input: &str) -> u32 {
    let (_, rucksacks) = parse_input(input).unwrap();

    rucksacks.iter().map(|r| priority(r.shared_item())).sum()
}

pub fn solve_part_2(input: &str) -> u32 {
    let (_, rucksacks) = parse_input(input).unwrap();

    let mut result = 0;

    for i in (0..rucksacks.len()).step_by(3) {
        let mut r1 = HashSet::<_>::from_iter(rucksacks[i].0.iter());
        let r2 = HashSet::<_>::from_iter(rucksacks[i + 1].0.iter());
        let r3 = HashSet::<_>::from_iter(rucksacks[i + 2].0.iter());

        r1.retain(|c| r2.contains(c) && r3.contains(c));

        let item = r1.iter().next().unwrap();

        result += priority(**item);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";

    #[test]
    fn test_solve_part_1_on_sample_input() {
        let solution = solve_part_1(SAMPLE_INPUT);

        assert_eq!(solution, 157);
    }

    #[test]
    fn test_solve_part_1_on_full_input() {
        let input = include_str!("../../input/2022/03.txt");

        let solution = solve_part_1(input);

        assert_eq!(solution, 7908);
    }

    #[test]
    fn test_solve_part_2_on_sample_input() {
        let solution = solve_part_2(SAMPLE_INPUT);

        assert_eq!(solution, 70);
    }

    #[test]
    fn test_solve_part_2_on_full_input() {
        let input = include_str!("../../input/2022/03.txt");

        let solution = solve_part_2(input);

        assert_eq!(solution, 2838);
    }
}
