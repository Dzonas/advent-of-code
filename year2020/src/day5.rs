#[cfg(test)]
mod tests {
    fn get_position(data: &[char], length: usize) -> usize {
        let mut left = 0.0;
        let mut right = length as f64 - 1.0;

        for c in data {
            if *c == 'F' || *c == 'L' {
                right = ((right - left) / 2.0).floor() + left;
            } else if *c == 'B' || *c == 'R' {
                left = ((right - left) / 2.0).ceil() + left;
            }
        }

        left as usize
    }

    fn solve_part_1(input: &str) -> usize {
        input
            .lines()
            .map(|boarding_pass| boarding_pass.chars().collect::<Vec<char>>())
            .map(|chars| (get_position(&chars[..7], 128), get_position(&chars[7..], 8)))
            .map(|(row, column)| row * 8 + column)
            .max()
            .unwrap()
    }

    fn solve_part_2(input: &str) -> usize {
        let mut ids_vec: Vec<usize> = input
            .lines()
            .map(|boarding_pass| boarding_pass.chars().collect::<Vec<char>>())
            .map(|chars| (get_position(&chars[..7], 128), get_position(&chars[7..], 8)))
            .map(|(row, column)| row * 8 + column)
            .collect();

        ids_vec.sort_unstable();
        let mut ids = ids_vec.iter();

        let mut prev_id = ids.next().unwrap();
        let mut my_id = 0;

        for curr_id in ids {
            if curr_id - prev_id == 2 {
                let id = prev_id + 1;
                let row = id / 8;

                if row != 0 && row != 127 {
                    my_id = id;
                }
            }
            prev_id = curr_id;
        }

        my_id
    }

    #[test]
    fn test_should_solve_part_1_preview() {
        let input = include_str!("../input/day5_test.txt");
        assert_eq!(solve_part_1(input), 820);
    }

    #[test]
    fn test_should_solve_part_1() {
        let input = include_str!("../input/day5.txt");
        assert_eq!(solve_part_1(input), 883);
    }

    #[test]
    fn test_should_solve_part_2() {
        let input = include_str!("../input/day5.txt");
        assert_eq!(solve_part_2(input), 532);
    }
}
