extern crate regex;

use std::fmt;
use regex::Regex;

struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32
}

impl Velocity {
    fn new(x: i32, y: i32) -> Velocity {
        Velocity { x, y }
    }
}

struct Star {
    pos: Position,
    vel: Velocity
}

impl Star {
    fn new(pos: Position, vel: Velocity) -> Star {
        Star { pos, vel }
    }

    ///
    /// Moves a star according to it's velocity.
    ///
    fn tick(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }

    ///
    /// Moves a star according to opposite of it's velocity.
    ///
    fn rev_tick(&mut self) {
        self.pos.x -= self.vel.x;
        self.pos.y -= self.vel.y;
    }
}

pub struct Sky {
    stars: Vec<Star>
}

impl Sky {
    fn new(stars: Vec<Star>) -> Sky {
        Sky { stars }
    }

    ///
    /// Ticks every star.
    ///
    pub fn tick(&mut self) {
        for star in &mut self.stars {
            star.tick();
        }
    }

    ///
    /// Does reverse tick for every star, that is undoes a tick.
    ///
    pub fn rev_tick(&mut self) {
        for star in &mut self.stars {
            star.rev_tick();
        }
    }

    pub fn tick_until_smallest_area(&mut self) -> u32 {
        let mut area = self.area();
        let mut t = 0;

        loop {
            self.tick();
            let temp = self.area();

            if temp < area {
                area = temp;
                t += 1;
            } else {
                self.rev_tick();
                break;
            }
        }

        t
    }

    ///
    /// Calculates width of the bounding box.
    /// Returns (width, maximal x, minimal x).
    ///
    fn width(&self) -> (usize, i32, i32) {
        let max_x = self.stars.iter().max_by_key(|x| x.pos.x).unwrap().pos.x;
        let min_x = self.stars.iter().min_by_key(|x| x.pos.x).unwrap().pos.x;

        (((max_x - min_x).abs() + 1) as usize,
            max_x,
            min_x)
    }

    ///
    /// Calculates height of the bounding box.
    /// Returns (height, maximal y, minimal y).
    ///
    fn height(&self) -> (usize, i32, i32) {
        let max_y = self.stars.iter().max_by_key(|x| x.pos.y).unwrap().pos.y;
        let min_y = self.stars.iter().min_by_key(|x| x.pos.y).unwrap().pos.y;

        (((max_y - min_y).abs() + 1) as usize,
         max_y,
         min_y)
    }

    ///
    /// Calculates area of the bounding box.
    ///
    fn area(&self) -> usize {
        self.height().0 * self.width().0
    }
}

impl fmt::Debug for Sky {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (width, _, min_x) = self.width();
        let (height, _, min_y) = self.height();
        let mut sky = vec!['.'; width * height];

        for star in &self.stars {
            let i = (star.pos.x - min_x) as usize;
            let j = (star.pos.y - min_y) as usize;

            let index = j * width + i;

            sky[index] = '#';
        }

        let mut sky_map = String::with_capacity(width * height + height - 1);

        for (i, cell) in sky.iter().enumerate() {
            sky_map.push(*cell);

            if (i + 1) as usize % width == 0 {
                sky_map.push('\n');
            }
        }

        write!(f, "{}", sky_map)
    }
}

pub fn parse(text: &str) -> Sky {
    let mut stars = Vec::with_capacity(text.lines().count());
    let re = Regex::new(r"(?m)position=<(?P<pos>.*?)> velocity=<(?P<vel>.*?)>").unwrap();

    for caps in re.captures_iter(text) {
        let p: Vec<i32> = caps["pos"].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();
        let v: Vec<i32> = caps["vel"].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();

        let pos = Position::new(p[0], p[1]);
        let vel = Velocity::new(v[0], v[1]);
        let star = Star::new(pos, vel);
        stars.push(star);
    }

    Sky::new(stars)
}