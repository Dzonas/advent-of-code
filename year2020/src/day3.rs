#[cfg(test)]
mod tests {
    fn solve_core(biome: &[Vec<char>], slope: (usize, usize)) -> usize {
        let height = biome.len();
        let width = biome[0].len();
        let mut pos = (0, 0);
        let mut counter = 0;

        loop {
            pos.0 += slope.0;
            pos.1 += slope.1;

            if pos.0 >= height {
                break;
            }
            if biome[pos.0][pos.1 % width] == '#' {
                counter += 1;
            }
        }

        counter
    }

    fn solve_part_1(input: &str) -> usize {
        let biome: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        solve_core(&biome, (1, 3))
    }

    fn solve_part_2(input: &str) -> usize {
        let biome: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

        slopes
            .iter()
            .map(|&slope| solve_core(&biome, slope))
            .product()
    }

    #[test]
    fn should_solve_part_1_preview() {
        let input = include_str!("../input/day3_test.txt");
        assert_eq!(solve_part_1(input), 7);
    }

    #[test]
    fn should_solve_part_1() {
        let input = include_str!("../input/day3.txt");
        assert_eq!(solve_part_1(input), 207);
    }

    #[test]
    fn should_solve_part_2_preview() {
        let input = include_str!("../input/day3_test.txt");
        assert_eq!(solve_part_2(input), 336);
    }

    #[test]
    fn should_solve_part_2() {
        let input = include_str!("../input/day3.txt");
        assert_eq!(solve_part_2(input), 2655892800);
    }
}
