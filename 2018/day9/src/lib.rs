use std::collections::VecDeque;
use std::mem;

#[derive(Debug)]
struct MarbleBoard {
    left: VecDeque<u32>,
    middle: u32,
    right: VecDeque<u32>,
}

impl MarbleBoard {
    fn new(size: usize, initial_value: u32) -> MarbleBoard {
        let left_size = size / 2;
        let right_size = size - left_size - 1;
        let left = VecDeque::with_capacity(left_size);
        let right = VecDeque::with_capacity(right_size);
        let middle = initial_value;

        MarbleBoard { left, middle, right }
    }

    ///
    /// Removes middle element. Moves middle position clockwise.
    ///
    fn remove(&mut self) -> u32 {
        let middle = self.middle;

        if self.right.is_empty() {
            self.middle = self.left.pop_front().unwrap();
        } else {
            self.middle = self.right.pop_front().unwrap();
        }

        middle
    }

    ///
    /// Inserts an element at the middle. Moves middle clockwise.
    ///
    fn insert(&mut self, v: u32) {
            self.right.push_front(self.middle);
            self.middle = v;
    }

    ///
    /// Moves middle clockwise i times.
    ///
    fn move_clockwise(mut self, mut n: usize) -> Self {
        if n > 0 {
            self.left.push_back(self.middle);
            self.middle = match self.right.pop_front() {
                Some(v) => v,
                None => {
                    mem::swap(&mut self.left, &mut self.right);
                    self.right.pop_front().unwrap()
                }
            };
            n -= 1;
            self.move_clockwise(n)
        } else {
            self
        }
    }

    ///
    /// Moves middle counter clockwise i times.
    ///
    fn move_counter_clockwise(mut self, mut n: usize) -> Self {
        if n > 0 {
            self.right.push_front(self.middle);
            self.middle = match self.left.pop_back() {
                Some(v) => v,
                None => {
                    mem::swap(&mut self.left, &mut self.right);
                    self.left.pop_back().unwrap()
                }
            };

            n -= 1;
            self.move_counter_clockwise(n)
        } else {
            self
        }
    }
}

pub struct MarbleGame {
    player_scores: Vec<u32>,
    n_marbles: usize,
}

impl MarbleGame {
    pub fn new(n_players: u32, n_marbles: usize) -> MarbleGame {
        let player_scores = vec![0; n_players as usize];

        MarbleGame { player_scores, n_marbles }
    }

    ///
    /// Calculates player scores by simulating the game.
    ///
    pub fn simulate(&mut self) {
        let n_players = self.player_scores.len();
        let mut board = MarbleBoard::new(self.n_marbles, 0);
        board.insert(1);

        for i in 2..self.n_marbles+1 {
            if i % 23 == 0 {
                let player = (i - 1) % n_players;

                board = board.move_counter_clockwise(7);
                let score = i as u32 + board.remove();
                self.player_scores[player] += score;
            } else {
                board = board.move_clockwise(2);
                board.insert(i as u32);
            }
        }
    }

    ///
    /// Returns currently highest score.
    ///
    pub fn get_high_score(&self) -> u32 {
        *self.player_scores.iter().max().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10_players_1618_marbles() {
        let mut marble_game = MarbleGame::new(10, 1618);
        marble_game.simulate();
        let high_score = marble_game.get_high_score();

        assert_eq!(8317, high_score);
    }

    #[test]
    fn test_13_players_7999_marbles() {
        let mut marble_game = MarbleGame::new(13, 7999);
        marble_game.simulate();
        let high_score = marble_game.get_high_score();

        assert_eq!(146373, high_score);
    }

    #[test]
    fn test_17_players_1104_marbles() {
        let mut marble_game = MarbleGame::new(17, 1104);
        marble_game.simulate();
        let high_score = marble_game.get_high_score();

        assert_eq!(2764, high_score);
    }

    #[test]
    fn test_21_players_6111_marbles() {
        let mut marble_game = MarbleGame::new(21, 6111);
        marble_game.simulate();
        let high_score = marble_game.get_high_score();

        assert_eq!(54718, high_score);
    }

    #[test]
    fn test_30_players_5807_marbles() {
        let mut marble_game = MarbleGame::new(30, 5807);
        marble_game.simulate();
        let high_score = marble_game.get_high_score();

        assert_eq!(37305, high_score);
    }
}