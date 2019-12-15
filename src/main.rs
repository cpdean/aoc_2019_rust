// failed day 10 solution
use std::collections::HashMap;
/// replace
use std::fs;
use std::hash::{Hash, Hasher};

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day10.txt")?;
    let asteroid_positions = parse_asteroid_field(&f);
    dbg!(asteroid_positions);
    Ok(())
}

pub fn parse_asteroid_field(s: &str) -> Vec<Point> {
    let lines = s.trim().split("\n");
    let mut asteroid_positions = vec![];
    for (y, line) in lines.enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                asteroid_positions.push(Point(x as f64, y as f64));
            }
        }
    }
    asteroid_positions
}

//type Point = (f64, f64);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(f64, f64);

impl std::cmp::Eq for Point {}

use std::mem;
fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (m, e, s) = integer_decode(self.0);
        m.hash(state);
        e.hash(state);
        s.hash(state);
        let (m2, e2, s2) = integer_decode(self.1);
        m2.hash(state);
        e2.hash(state);
        s2.hash(state);
    }
}

fn slope(a: &Point, b: &Point) -> f64 {
    let Point(a_x, a_y) = a;
    let Point(b_x, b_y) = b;
    (b_y - a_y) / (b_x - a_x)
}

fn distance(a: &Point, b: &Point) -> f64 {
    let Point(a_x, a_y) = a;
    let Point(b_x, b_y) = b;
    let eh = (b_x - a_x).exp2() + (b_y - a_y).exp2();
    eh.sqrt()
}

pub fn points_are_on_a_line(a: &Point, b: &Point, c: &Point) -> bool {
    let ab = slope(a, b);
    let ac = slope(a, c);
    let bc = slope(b, c);
    ab == ac && ac == bc
}

pub fn identity_vector(a: &Point) -> Point {
    let d = distance(&Point(0.0, 0.0), a);
    let Point(x, y) = a;
    Point(x / d, y / d)
}

pub fn subtract(a: &Point, b: &Point) -> Point {
    let Point(a_x, a_y) = a;
    let Point(b_x, b_y) = b;
    Point(a_x - b_x, a_y - b_y)
}

pub fn score_of_base(possible_base: &Point, mut asteroids: Vec<Point>) -> usize {
    asteroids.sort_by(|a, b| {
        let a_distance = distance(&possible_base, a);
        let b_distance = distance(&possible_base, b);
        a_distance.partial_cmp(&b_distance).unwrap()
    });
    let mut non_blocked_asteroids = HashMap::new();
    for p in asteroids {
        let identity = identity_vector(&subtract(possible_base, &p));
        let mut p_is_blocked = false;
        for asteroid in &non_blocked_asteroids {
            if non_blocked_asteroids.contains_key(&identity) {
                p_is_blocked = true;
                break;
            }
        }
        if !p_is_blocked {
            non_blocked_asteroids.insert(identity, p);
        }
    }
    non_blocked_asteroids.len()
}

pub fn find_best_station(positions: Vec<Point>) -> (Point, usize) {
    let mut best_so_far = (Point(0.0, 0.0), 0);
    for i in 0..positions.len() {
        let possible_base = positions[i];
        let head = &positions[0..i];
        let tail = &positions[i + 1..];
        let ordered: Vec<Point> = [head, tail].concat();
        let base_score = score_of_base(&possible_base, ordered);
        let (_, score) = best_so_far;
        if score < (base_score) {
            best_so_far = dbg!((possible_base, base_score));
        }
    }
    best_so_far
}

#[cfg(test)]
mod tests {
    use super::*;
    fn example1() -> &'static str {
        ".#..#
.....
#####
....#
...##"
    }

    #[test]
    fn first_example() {
        let positions = parse_asteroid_field(example1());
        assert_eq!(positions.len(), 10);
        let best_point = find_best_station(positions);
        assert_eq!(best_point, (Point(3.0, 4.0), 8));
    }

    #[test]
    fn first_tiny() {
        let positions = vec![Point(0.0, 0.0), Point(0.0, 1.0), Point(0.0, 2.0)];
        let best_point = find_best_station(positions);
        assert_eq!(best_point, (Point(0.0, 1.0), 2));
    }
}
