use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn manhattan_distance(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

///
/// Each coordinate has a closest given point.
/// This struct contains that point id, distance to it and if any other
/// point is equally close to it.
///
#[derive(Debug, Clone)]
struct ClosestPoint {
    id: u32,
    distance: u32,
    double: bool,
}

impl ClosestPoint {
    pub fn new(id: u32, distance: u32, double: bool) -> ClosestPoint {
        ClosestPoint { id, distance, double }
    }
}


///
/// Represents a grid, on which all points are placed.
///
#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    points: HashMap<u32, Point>,
}

impl Grid {
    ///
    /// Points is a HashMap where the key is the point id
    /// and the value is the coordinate of that point.
    ///
    pub fn new(points: HashMap<u32, Point>) -> Grid {
        let width = (points.iter().max_by_key(|(_, v)| v.x).unwrap().1.x + 1) as usize;
        let height = (points.iter().max_by_key(|(_, v)| v.y).unwrap().1.y + 1) as usize;

        Grid { width, height, points }
    }

    ///
    /// Calculates which point is represented by the given index.
    ///
    fn get_point(&self, index: usize) -> Point {
        let x = (index % self.width) as i32;
        let y = (index / self.width) as i32;

        Point::new(x, y)
    }

    ///
    /// For each coordinate it calculates which point is the closest.
    /// Returns Vec where each element represents which point
    /// is the closest for each coordinate.
    ///
    fn get_closest_point_matrix(&self) -> Vec<ClosestPoint> {
        let mut distances: Vec<Option<ClosestPoint>> = vec![None; self.height * self.width];

        for i in 0..(self.width * self.height) {
            let point = self.get_point(i);

            for (given_point_id, given_point) in &self.points {
                let dist = point.manhattan_distance(given_point);

                match &mut distances[i] {
                    Some(a) => {
                        if dist < a.distance {
                            a.distance = dist;
                            a.id = *given_point_id;
                            a.double = false;
                        } else if dist == a.distance && a.id != *given_point_id {
                            a.double = true;
                        }
                    }
                    None => distances[i] = Some(ClosestPoint::new(*given_point_id, dist, false))
                }
            }
        }

        distances.into_iter().map(|f|f.unwrap()).collect()
    }

    ///
    /// Calculates what is the biggest area that isn't infinite
    /// and it "belongs" to a certain point - doesn't "belong" to multiple points.
    /// To "belong" to given point means that that given point is the closest.
    ///
    pub fn biggest_non_infinite_area(&self) -> usize {
        let distances = self.get_closest_point_matrix(); // Get closest point for each coordinate.
        let finite_area_ids: HashSet<u32> = HashSet::from_iter(self.points.keys().map(|k| *k)); // Create set of all points id.
        let infinite_area_ids = self.border_points(&distances); // Get set of all points that are on the border of the grid.

        // Calculate area size for each finite area.
        let mut areas = Vec::with_capacity(finite_area_ids.len());
        for id in finite_area_ids.difference(&infinite_area_ids) {
            areas.push(self.get_area(&distances, *id));
        }

        *areas.iter().max().unwrap()
    }

    ///
    /// Calculates which points are on the border.
    /// Returns set of ids of these points.
    ///
    fn border_points(&self, distances: &Vec<ClosestPoint>) -> HashSet<u32> {
        let mut infinite_area_ids = HashSet::new();

        for (i, cp) in distances.iter().enumerate() {
            let point = self.get_point(i);

            if cp.double == false && (point.x == 0 || point.x == (self.width - 1) as i32 || point.y == 0 || point.y == (self.height - 1) as i32) {
                infinite_area_ids.insert(cp.id);
            }
        }

        infinite_area_ids
    }

    ///
    /// Gets size of the area whose coordinates are closest to given point.
    ///
    fn get_area(&self, distances: &Vec<ClosestPoint>, point_id: u32) -> usize {
        distances.iter()
            .filter(|cp| cp.id == point_id && cp.double == false)
            .count()
    }

    ///
    /// Calculates how big is the area on the grid
    /// where sum of distances to each given point
    /// is smaller than certain distance.
    ///
    pub fn area_with_equal_distance(&self, distance: u32) -> usize {
        let mut area = 0;

        for i in 0..(self.width * self.height) {
            let point = self.get_point(i);
            let mut sum_dist = 0;

            for (_, given_point) in &self.points {
                sum_dist += given_point.manhattan_distance(&point);
            }

            if sum_dist < distance {
                area += 1;
            }
        }

        area
    }
}

///
/// Parses input.
/// Outputs a HashMap where key is the point id
/// and the value is the coordinate of that point.
///
pub fn parse(text: &str) -> HashMap<u32, Point> {
    let mut points = HashMap::new();

    for (i, line) in text.lines().enumerate() {
        let line: Vec<&str> = line.trim().split(", ").collect();
        let x: i32 = line[0].parse().unwrap();
        let y: i32 = line[1].parse().unwrap();
        let point = Point::new(x, y);

        points.insert(i as u32, point);
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../test");

    #[test]
    fn test_manhattan_distance() {
        let point1 = Point::new(2, 3);
        let point2 = Point::new(-3, 1);

        assert_eq!(7, point1.manhattan_distance(&point2));
    }

    #[test]
    fn test_largest_area() {
        let points = parse(INPUT);
        let grid = Grid::new(points);

        assert_eq!(17, grid.biggest_non_infinite_area());
    }

    #[test]
    fn test_area_with_equal_distance() {
        let points = parse(INPUT);
        let grid = Grid::new(points);

        assert_eq!(16, grid.area_with_equal_distance(32));
    }
}