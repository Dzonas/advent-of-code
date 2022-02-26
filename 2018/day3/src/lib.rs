extern crate regex;

use self::regex::Regex;

pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 1000;

#[derive(Debug)]
struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Size {
        Size { width, height }
    }
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

#[derive(Debug)]
pub struct Claim {
    id: u32,
    margin: Position,
    size: Size,
}

impl Claim {
    pub fn new(id: u32, left_margin: usize, top_margin: usize, width: usize, height: usize) -> Claim {
        let margin = Position::new(left_margin, top_margin);
        let size = Size::new(width, height);

        Claim { id, margin, size }
    }

    pub fn parse_input(input: &str) -> Vec<Claim> {
        let re = Regex::new(r"(?m)#(?P<id>[0-9]+) @ (?P<left_margin>[0-9]+),(?P<right_margin>[0-9]+): (?P<width>[0-9]+)x(?P<height>[0-9]+)").unwrap();
        let mut claims = Vec::new();

        for capture in re.captures_iter(input) {
            let id = capture["id"].parse().unwrap();
            let left_margin = capture["left_margin"].parse().unwrap();
            let right_margin = capture["right_margin"].parse().unwrap();
            let width = capture["width"].parse().unwrap();
            let height = capture["height"].parse().unwrap();

            claims.push(Claim::new(id, left_margin, right_margin, width, height));
        }

        claims
    }

    pub fn get_positions(&self) -> Vec<Position> {
        let mut positions = Vec::with_capacity(self.size.width * self.size.height);

        for i in self.margin.y..(self.margin.y + self.size.height) {
            for j in self.margin.x..(self.margin.x + self.size.width) {
                positions.push(Position::new(j, i));
            }
        }

        positions
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
struct Matrix {
    size: Size,
    data: Vec<u32>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Matrix {
        let size = Size::new(width, height);
        let data = vec![0; width * height];

        Matrix { size, data }
    }

    pub fn get(&self, y: usize, x: usize) -> u32 {
        self.data[&self.size.width * y + x]
    }

    pub fn set(&mut self, y: usize, x: usize, data: u32) {
        self.data[self.size.width * y + x] = data;
    }

    pub fn get_data(&self) -> &Vec<u32> {
        &self.data
    }
}

#[derive(Debug)]
pub struct Fabric {
    size: Size,
    claims: Vec<Claim>,
}

impl Fabric {
    pub fn new(width: usize, height: usize, claims: Vec<Claim>) -> Fabric {
        let size = Size::new(width, height);

        Fabric { size, claims }
    }

    ///
    /// Calculates matrix, which contains how many claims have
    /// claimed specific square inch of the fabric.
    ///
    fn get_claim_matrix(&self) -> Matrix {
        // Create matrix, that contains number of claims for each square inch
        // of the fabric
        let mut data = Matrix::new(self.size.width, self.size.height);

        for claim in &self.claims {
            let positions = claim.get_positions();

            for pos in positions {
                let current = data.get(pos.y, pos.x);
                data.set(pos.y, pos.x, current + 1);
            }
        }

        data
    }

    ///
    /// Calculates, how many square inches are overlapped
    /// by the claims at least n times.
    ///
    pub fn overlap_size(&self, n: u32) -> usize {
        let claim_matrix = self.get_claim_matrix();
        // Get claims which are claimed by n claims or more
        let at_least_two: Vec<&u32> = claim_matrix.get_data().iter().filter(|v| **v >= n).collect();

        at_least_two.len()
    }

    ///
    /// Calculates which (if any) claim is not overlapping with any another claim.
    /// Returns not overlapping claim.
    ///
    pub fn get_not_overlapping_claim(&self) -> Option<&Claim> {
        let claim_matrix = self.get_claim_matrix();
        let mut no_overlapped = None; // Resulting claim

        for claim in &self.claims {
            let positions = claim.get_positions();
            let mut found = true;

            for position in positions {
                if claim_matrix.get(position.y, position.x) != 1 {
                    found = false;
                    break;
                }
            }

            if found {
                no_overlapped = Some(claim);
                break;
            }
        }

        no_overlapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_size() {
        let input = include_str!("../test");
        let claims = Claim::parse_input(input);
        let fabric = Fabric::new(WIDTH, HEIGHT, claims);

        assert_eq!(4, fabric.overlap_size(2));
    }

    #[test]
    fn test_overlap() {
        let input = include_str!("../test");
        let claims = Claim::parse_input(input);
        let fabric = Fabric::new(WIDTH, HEIGHT, claims);
        let not_overlapped_claim = fabric.get_not_overlapping_claim();

        assert_eq!(3, not_overlapped_claim.unwrap().id());
    }

    #[test]
    fn test_claim_position() {
        let claim = Claim::new(3, 5, 5, 2, 2);
        let positions = claim.get_positions();
        let true_positions = vec![
            Position::new(5, 5),
            Position::new(6, 5),
            Position::new(5, 6),
            Position::new(6, 6)
        ];

        assert_eq!(true_positions, positions);
    }
}
