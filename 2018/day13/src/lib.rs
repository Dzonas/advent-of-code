use std::iter::Cycle;
use std::vec::IntoIter;
use std::fmt;
use std::ops::{Mul, MulAssign};

const CART_SPEED: i32 = 1;

#[derive(Copy, Clone)]
enum Track {
    StraightVertical,
    StraightHorizontal,
    TurnSlash,
    TurnBackSlash,
    Intersection,
    None
}

impl Track {
    fn value(&self) -> char {
        match self {
            Track::StraightVertical => '|',
            Track::StraightHorizontal => '-',
            Track::TurnSlash => '/',
            Track::TurnBackSlash => '\\',
            Track::Intersection => '+',
            Track::None => ' '
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn value(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<'
        }
    }
}

#[derive(Clone)]
enum Rotation {
    Clockwise,
    CounterClockwise,
    None
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Complex {
    real: i32,
    imaginary: i32
}

impl Complex {
    fn new(real: i32, imaginary: i32) -> Complex {
        Complex { real, imaginary }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let k1 = other.real * (self.real + self.imaginary);
        let k2 = self.real * (other.imaginary - other.real);
        let k3 = self.imaginary * (other.real + other.imaginary);

        Complex::new(k1 - k3, k1 + k2)
    }
}

impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, other: Complex) {
        *self = *self * other;
    }
}

struct Cart {
    position: Position,
    direction: Complex,
    states: Cycle<IntoIter<Rotation>>
}

impl Cart {
    fn new(position: Position, direction: Complex) -> Cart {
        let states = vec![Rotation::CounterClockwise, Rotation::None, Rotation::Clockwise].into_iter().cycle();

        Cart { position, direction, states }
    }

    ///
    /// Moves cart forward according to it's direction.
    ///
    fn move_cart(&mut self) {
        self.position.x = self.position.x + (self.direction.real * CART_SPEED);
        self.position.y = self.position.y + (self.direction.imaginary * CART_SPEED);
    }


    ///
    /// Turns cart clockwise or counterclockwise by changing it's direction.
    ///
    fn turn(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Clockwise => {
                self.direction *= Complex::new(0, 1);
            },
            Rotation::CounterClockwise => {
                self.direction *= Complex::new(0, -1);
            },
            Rotation::None => ()
        }
    }

    ///
    /// Should be called when cart is on the intersection.
    /// Turns cart according to it's internal state.
    ///
    fn intersection(&mut self) {
        let rotation = self.states.next().unwrap();
        self.turn(rotation);
    }

    fn update(&mut self, track_part: &Track) {
        let cart_direction = self.direction();

        match *track_part {
            Track::StraightVertical => (),
            Track::StraightHorizontal => (),
            Track::TurnSlash => {
                if cart_direction == Direction::Up || cart_direction == Direction::Down {
                    self.turn(Rotation::Clockwise);
                } else {
                    self.turn(Rotation::CounterClockwise);
                }
            },
            Track::TurnBackSlash => {
                if cart_direction == Direction::Up || cart_direction == Direction::Down {
                    self.turn(Rotation::CounterClockwise);
                } else {
                    self.turn(Rotation::Clockwise);
                }
            },
            Track::Intersection => self.intersection(),
            _ => panic!("Cart is not on track! {:?}", self.position)
        }
    }

    fn direction(&self) -> Direction {
        let direction;
        if self.direction.real == 0 {
            if self.direction.imaginary == 1 {
                direction = Direction::Down;
            } else if self.direction.imaginary == -1 {
                direction = Direction::Up;
            } else {
                panic!("Unknown direction");
            }
        } else if self.direction.imaginary == 0 {
            if self.direction.real == 1 {
                direction = Direction::Right;
            } else if self.direction.real == -1 {
                direction = Direction::Left;
            } else {
                panic!("Unknown direction");
            }
        } else {
            panic!("Unknown direction");
        }

        direction
    }
}

pub struct TrackSystem {
    tracks: Vec<Track>,
    carts: Vec<Cart>,
    width: usize,
    height: usize,
}

impl TrackSystem {
    fn new(width: usize, height: usize, track_parts: Vec<(Position, Track)>, carts: Vec<Cart>) -> TrackSystem {
        let mut tracks = vec![Track::None; width * height];

        for (pos, part) in track_parts {
            let i = TrackSystem::get_index(width, &pos);
            tracks[i] = part;
        }

        TrackSystem { tracks, carts, width, height }
    }

    ///
    /// Calculates index in vec based on 2D position and width of the matrix.
    ///
    fn get_index(width: usize, pos: &Position) -> usize {
        pos.y as usize * width + pos.x as usize
    }

    pub fn simulate_until_crash(&mut self) -> Position {
        let TrackSystem { ref tracks, ref mut carts, ref width, height: _ } = *self;
        let pos;

        'outer: loop {
            carts.sort_by_key(|f| (f.position.y, f.position.x));

            for n in 0..carts.len() {
                carts[n].move_cart();

                for i in 0..carts.len() {
                    for j in i + 1..carts.len() {
                        if carts[i].position == carts[j].position {
                            pos = carts[i].position.clone();
                            break 'outer;
                        }
                    }
                }

                let resulting_track_part = &tracks[TrackSystem::get_index(*width, &carts[n].position)];
                carts[n].update(resulting_track_part);
            }
        }

        pos
    }

    pub fn simulate_until_last(&mut self) -> Position {
        let TrackSystem { ref tracks, ref mut carts, ref width, height: _ } = *self;
        let pos;

        'outer: loop {
            carts.sort_by_key(|f| (f.position.y, f.position.x));

            let mut end = carts.len();
            let mut n = 0;

            while n < end {
                carts[n].move_cart();
                let resulting_track_part = &tracks[TrackSystem::get_index(*width, &carts[n].position)];
                carts[n].update(resulting_track_part);

                let mut change = false;
                let mut removed_cart_pos = Position::new(0, 0);

                for i in 0..carts.len() {
                    if carts[i].position == carts[n].position && n != i {
                        removed_cart_pos = carts[i].position;
                        carts.remove(n);

                        if i > n {
                            carts.remove(i - 1);
                        } else {
                            carts.remove(i);
                            n -= 1;
                        }
                        end = carts.len();
                        change = true;
                        break;
                    }
                }

                if carts.len() == 1 {
                    let mut cart = carts.pop().unwrap();

                    if TrackSystem::get_index(*width, &removed_cart_pos) < TrackSystem::get_index(*width, &cart.position) {
                        cart.move_cart();
                    }

                    pos = cart.position;
                    break 'outer;
                }

                if !change {
                    n += 1;
                }
            }
        }

        pos
    }
}

impl fmt::Debug for TrackSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let mut output = String::with_capacity((self.width + 1)  * self.height);
        let mut output= Vec::with_capacity((self.width + 1) * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let i = TrackSystem::get_index(self.width, &Position::new(x as i32, y as i32));
                output.push(self.tracks[i].value());
            }

            output.push('\n');
        }
        output.pop();

        for cart in &self.carts {
            let i = TrackSystem::get_index(self.width + 1, &cart.position);
            output[i] = cart.direction().value();
        }

        let output_string: String = output.iter().collect();

        write!(f, "{}", output_string)
    }
}

pub fn parse(text: &str) -> TrackSystem {
    let width = text.lines().map(|f| f.len()).max().unwrap();
    let height = text.lines().count();

    let mut track_parts = Vec::with_capacity(width * height);
    let mut carts = Vec::new();

    for (i, line) in text.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let pos = Position::new(j as i32, i as i32);

            let track_part = match c {
                '/' => Track::TurnSlash,
                '\\' => Track::TurnBackSlash,
                '+' => Track::Intersection,
                '|' => Track::StraightVertical,
                '^' => {
                    carts.push(Cart::new(pos, Complex::new(0, -1)));
                    Track::StraightVertical
                },
                'v' => {
                    carts.push(Cart::new(pos, Complex::new(0, 1)));
                    Track::StraightVertical
                },
                '-' => Track::StraightHorizontal,
                '>' => {
                    carts.push(Cart::new(pos, Complex::new(1, 0)));
                    Track::StraightHorizontal
                },
                '<' => {
                    carts.push(Cart::new(pos, Complex::new(-1, 0)));
                    Track::StraightHorizontal
                },
                ' ' => Track::None,
                _ => panic!("Unknown track parts {}", c)
            };

            track_parts.push((pos, track_part));
        }
    }

    track_parts.shrink_to_fit();

    TrackSystem::new(width, height, track_parts, carts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cart_rotation_on_intersection() {
        let mut cart = Cart::new(Position::new(0, 0), Complex::new(0, -1));
        cart.intersection();
        assert_eq!(Direction::Left, cart.direction());
        cart.intersection();
        assert_eq!(Direction::Left, cart.direction());
        cart.intersection();
        assert_eq!(Direction::Up, cart.direction());
    }

    #[test]
    fn test_track_system() {
        let input = include_str!("../test");
        let mut track_system = parse(input);
        let crash_position = track_system.simulate_until_crash();

        assert_eq!(Position::new(7, 3), crash_position);
    }

    #[test]
    fn test_complex_multiplication() {
        let a = Complex::new(2, -2);
        let b = Complex::new(-5, 3);
        let c = a * b;

        assert_eq!(Complex::new(-4, 16), c);
    }

    #[test]
    fn test_track_system_with_removal() {
        let input = include_str!("../test2");
        let mut track_system = parse(input);
        let last_cart_standing = track_system.simulate_until_last();

        assert_eq!(Position::new(6, 4), last_cart_standing);
    }
}