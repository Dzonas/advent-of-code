use nom::{
    character::complete::{char, newline, one_of},
    multi::many1,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn next(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn prev(&self) -> Shape {
        self.next().next()
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

fn parse_first_column(input: &str) -> IResult<&str, Shape> {
    let (input, output) = one_of("ABC")(input)?;

    let shape = match output {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => unreachable!(),
    };

    Ok((input, shape))
}

fn parse_second_column_part_1(input: &str) -> IResult<&str, Shape> {
    let (input, output) = one_of("XYZ")(input)?;

    let shape = match output {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => unreachable!(),
    };

    Ok((input, shape))
}
type Strategy1 = (Shape, Shape);

fn parse_strategy_part_1(input: &str) -> IResult<&str, Strategy1> {
    let (input, left) = parse_first_column(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, right) = parse_second_column_part_1(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (left, right)))
}

fn parse_part_1(input: &str) -> IResult<&str, Vec<Strategy1>> {
    many1(parse_strategy_part_1)(input)
}

fn play_round(player: Shape, opponent: Shape) -> Outcome {
    if player == opponent {
        Outcome::Draw
    } else if player == opponent.next() {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}

fn score_round(player: Shape, opponent: Shape) -> u32 {
    let outcome = play_round(player, opponent);

    player.score() + outcome.score()
}

pub fn solve_part_1(input: &str) -> u32 {
    let (_, data) = parse_part_1(input).unwrap();

    data.into_iter()
        .map(|(opponent, player)| score_round(player, opponent))
        .sum()
}

fn parse_second_column_part_2(input: &str) -> IResult<&str, Outcome> {
    let (input, output) = one_of("XYZ")(input)?;

    let outcome = match output {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => unreachable!(),
    };

    Ok((input, outcome))
}

type Strategy2 = (Shape, Outcome);

fn parse_strategy_part_2(input: &str) -> IResult<&str, Strategy2> {
    let (input, left) = parse_first_column(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, right) = parse_second_column_part_2(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (left, right)))
}

fn parse_part_2(input: &str) -> IResult<&str, Vec<Strategy2>> {
    many1(parse_strategy_part_2)(input)
}

fn get_shape_to_play(opponent: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Outcome::Win => opponent.next(),
        Outcome::Draw => opponent,
        Outcome::Lose => opponent.prev(),
    }
}

fn get_score_to_achieve(opponent: Shape, outcome: Outcome) -> u32 {
    let player = get_shape_to_play(opponent, outcome);

    player.score() + outcome.score()
}

pub fn solve_part_2(input: &str) -> u32 {
    let (_, data) = parse_part_2(input).unwrap();

    data.into_iter()
        .map(|(opponent, outcome)| get_score_to_achieve(opponent, outcome))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "A Y\nB X\nC Z\n";

    #[test]
    fn test_solve_part_1_on_sample_input() {
        let solution = solve_part_1(SAMPLE_INPUT);

        assert_eq!(solution, 15);
    }

    #[test]
    fn test_solve_part_1_on_full_input() {
        let input = include_str!("../../input/2022/02.txt");

        let solution = solve_part_1(input);

        assert_eq!(solution, 14827);
    }

    #[test]
    fn test_solve_part_2_on_sample_input() {
        let solution = solve_part_2(SAMPLE_INPUT);

        assert_eq!(solution, 12);
    }

    #[test]
    fn test_solve_part_2_on_full_input() {
        let input = include_str!("../../input/2022/02.txt");

        let solution = solve_part_2(input);

        assert_eq!(solution, 13889);
    }
}
