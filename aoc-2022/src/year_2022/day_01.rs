use nom::{
    character::complete::{digit1, newline},
    combinator::opt,
    multi::many1,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, calories) = digit1(input)?;
    let (input, _) = newline(input)?;
    let calories = calories.parse().unwrap();
    Ok((input, calories))
}

fn parse_elf(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, elf_calories) = many1(parse_line)(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((input, elf_calories))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, elves_calories) = many1(parse_elf)(input)?;

    Ok((input, elves_calories))
}

pub fn solve_part_1(input: &str) -> u32 {
    let (_, elves_calories) = parse_input(input).unwrap();

    elves_calories.iter().map(|v| v.iter().sum()).max().unwrap()
}

pub fn solve_part_2(input: &str) -> u32 {
    let (_, total_calories) = parse_input(input).unwrap();

    let mut per_elf_calories: Vec<u32> = total_calories
        .iter()
        .map(|elf_calories| elf_calories.iter().sum())
        .collect();
    per_elf_calories.sort();
    per_elf_calories.reverse();

    per_elf_calories.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str =
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn test_solve_part_1_on_sample_input() {
        let solution = solve_part_1(SAMPLE_INPUT);

        assert_eq!(solution, 24000);
    }

    #[test]
    fn test_solve_part_1_on_full_input() {
        let input = include_str!("../../input/2022/01.txt");

        let solution = solve_part_1(&input);

        assert_eq!(solution, 71124);
    }

    #[test]
    fn test_solve_part_2_on_sample_input() {
        let solution = solve_part_2(SAMPLE_INPUT);

        assert_eq!(solution, 45000);
    }

    #[test]
    fn test_solve_part_2_on_full_input() {
        let input = include_str!("../../input/2022/01.txt");

        let solution = solve_part_2(input);

        assert_eq!(solution, 204639);
    }
}
