use nom::{
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    multi::many1,
    IResult,
};

struct Range {
    left: u32,
    right: u32,
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        (other.left <= self.left && self.left <= other.right)
            || (other.left <= self.right && self.right <= other.right)
            || (self.left <= other.left && other.left <= self.right)
            || (self.left <= other.right && other.right <= self.right)
    }

    fn contains(&self, other: &Range) -> bool {
        self.left <= other.left && other.right <= self.right
    }
}

type AssignmentPair = (Range, Range);

fn parse_assignment(input: &str) -> IResult<&str, Range> {
    let (input, left) = map_res(digit1, |s: &str| s.parse())(input)?;
    let (input, _) = char('-')(input)?;
    let (input, right) = map_res(digit1, |s: &str| s.parse())(input)?;

    let range = Range { left, right };

    Ok((input, range))
}

fn parse_line(input: &str) -> IResult<&str, AssignmentPair> {
    let (input, assignment1) = parse_assignment(input)?;
    let (input, _) = char(',')(input)?;
    let (input, assignment2) = parse_assignment(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, (assignment1, assignment2)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<AssignmentPair>> {
    many1(parse_line)(input)
}

pub fn solve_part_1(input: &str) -> u32 {
    let (_, assignments) = parse_input(input).unwrap();

    assignments
        .into_iter()
        .filter(|(left, right)| left.contains(right) || right.contains(left))
        .map(|_| 1)
        .sum()
}

pub fn solve_part_2(input: &str) -> u32 {
    let (_, assignments) = parse_input(input).unwrap();

    assignments
        .into_iter()
        .filter(|(left, right)| left.overlaps(right))
        .map(|_| 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";

    #[test]
    fn test_solve_part_1_on_sample_input() {
        let solution = solve_part_1(INPUT);

        assert_eq!(solution, 2);
    }

    #[test]
    fn test_solve_part_1_on_full_input() {
        let input = include_str!("../../input/2022/04.txt");

        let solution = solve_part_1(input);

        assert_eq!(solution, 496);
    }

    #[test]
    fn test_solve_part_2_on_sample_input() {
        let solution = solve_part_2(INPUT);

        assert_eq!(solution, 4);
    }

    #[test]
    fn test_solve_part_2_on_full_input() {
        let input = include_str!("../../input/2022/04.txt");

        let solution = solve_part_2(input);

        assert_eq!(solution, 847);
    }
}
