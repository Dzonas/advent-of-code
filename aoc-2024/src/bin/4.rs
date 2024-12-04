fn main() {
    let input = include_str!("../../inputs/4.txt");

    println!("{}", solve_part_1(input));
    println!("{}", solve_part_2(input));
}

fn solve_part_1(input: &str) -> u32 {
    let v: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = v.len();
    let width = v[0].len();

    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            if j <= width - 4
                && v[i][j] == 'X'
                && v[i][j + 1] == 'M'
                && v[i][j + 2] == 'A'
                && v[i][j + 3] == 'S'
            {
                sum += 1;
            }
            if j >= 3 && v[i][j - 3..=j] == ['S', 'A', 'M', 'X'] {
                sum += 1;
            }
            if i <= width - 4
                && v[i][j] == 'X'
                && v[i + 1][j] == 'M'
                && v[i + 2][j] == 'A'
                && v[i + 3][j] == 'S'
            {
                sum += 1;
            }
            if i >= 3
                && v[i][j] == 'X'
                && v[i - 1][j] == 'M'
                && v[i - 2][j] == 'A'
                && v[i - 3][j] == 'S'
            {
                sum += 1;
            }
            if j <= width - 4
                && i >= 3
                && v[i][j] == 'X'
                && v[i - 1][j + 1] == 'M'
                && v[i - 2][j + 2] == 'A'
                && v[i - 3][j + 3] == 'S'
            {
                sum += 1;
            }
            if j <= width - 4
                && i <= width - 4
                && v[i][j] == 'X'
                && v[i + 1][j + 1] == 'M'
                && v[i + 2][j + 2] == 'A'
                && v[i + 3][j + 3] == 'S'
            {
                sum += 1;
            }
            if j >= 3
                && i >= 3
                && v[i][j] == 'X'
                && v[i - 1][j - 1] == 'M'
                && v[i - 2][j - 2] == 'A'
                && v[i - 3][j - 3] == 'S'
            {
                sum += 1;
            }
            if j >= 3
                && i <= width - 4
                && v[i][j] == 'X'
                && v[i + 1][j - 1] == 'M'
                && v[i + 2][j - 2] == 'A'
                && v[i + 3][j - 3] == 'S'
            {
                sum += 1;
            }
        }
    }

    sum
}

fn solve_part_2(input: &str) -> u32 {
    let v: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = v.len();
    let width = v[0].len();

    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            if i > 0
                && i < width - 1
                && j > 0
                && j < height - 1
                && ((v[i - 1][j - 1] == 'M' && v[i][j] == 'A' && v[i + 1][j + 1] == 'S')
                    || (v[i - 1][j - 1] == 'S' && v[i][j] == 'A' && v[i + 1][j + 1] == 'M'))
                && ((v[i + 1][j - 1] == 'M' && v[i][j] == 'A' && v[i - 1][j + 1] == 'S')
                    || (v[i + 1][j - 1] == 'S' && v[i][j] == 'A' && v[i - 1][j + 1] == 'M'))
            {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "MMMSXXMASM\n\
				 MSAMXMSMSA\n\
				 AMXSXMAAMM\n\
				 MSAMASMSMX\n\
				 XMASAMXAMM\n\
				 XXAMMXXAMA\n\
				 SMSMSASXSS\n\
				 SAXAMASAAA\n\
				 MAMMMXMMMM\n\
				 MXMXAXMASX";

    #[test]
    fn test_solve_part_1() {
        let result = solve_part_1(INPUT);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_solve_part_2() {
        let result = solve_part_2(INPUT);

        assert_eq!(result, 9);
    }
}
