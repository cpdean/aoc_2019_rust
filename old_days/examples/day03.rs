use std::collections::{HashMap, HashSet};
use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

#[derive(Debug, PartialEq)]
pub enum Path {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day03.txt")?;
    let input_state: Vec<Vec<((i32, i32), i32)>> = f
        .trim()
        .split("\n")
        .map(to_wire)
        .map(wire_to_trail)
        .map(|trail| {
            let mut trail_with_distance = vec![];
            let mut distance = 0;
            for t in trail {
                trail_with_distance.push((t, distance));
                distance += 1;
            }
            trail_with_distance
        })
        .collect();
    let a = &input_state[0];
    let b = &input_state[1];
    let mut a_set = HashSet::new();
    let mut b_set = HashSet::new();
    for _a in a {
        a_set.insert(_a.0);
    }

    for _b in b {
        b_set.insert(_b.0);
    }
    let intersection = a_set.intersection(&b_set).clone();
    let mut min = 100000;
    for i in intersection.clone() {
        if *i == (0, 0) {
            continue;
        }
        let (x, y) = i;
        let distance = x.abs() + y.abs();
        if min > distance {
            min = distance;
        }
    }
    dbg!(min);
    let mut a_set_step = HashMap::new();
    let mut b_set_step = HashMap::new();
    for _a in a {
        if a_set_step.contains_key(&_a.0) {
            continue;
        }
        a_set_step.insert(_a.0, _a.1);
    }

    for _b in b {
        if b_set_step.contains_key(&_b.0) {
            continue;
        }
        b_set_step.insert(_b.0, _b.1);
    }

    let mut step_min = 1000000000;
    for i in intersection {
        let a_step = a_set_step.get(i).unwrap();
        let b_step = b_set_step.get(i).unwrap();
        if a_step + b_step == 0 {
            continue;
        }
        step_min = if step_min > a_step + b_step {
            dbg!((a_step + b_step));
            a_step + b_step
        } else {
            step_min
        };
    }
    dbg!(step_min);

    // minimize steps to the intersection, and then find the intersection with the lowest sum of
    // steps
    Ok(())
}

pub fn to_wire(line: &str) -> Vec<Path> {
    line.split(",")
        .map(|e| {
            use Path::*;
            let (head, rest) = (e.get(0..1).unwrap(), e.get(1..).unwrap());
            let num: i32 = rest.parse().unwrap();
            match (head, num) {
                ("U", x) => Up(x),
                ("R", x) => Right(x),
                ("D", x) => Down(x),
                ("L", x) => Left(x),
                (h, x) => panic!("got '{}' '{}'", h, x),
            }
        })
        .collect()
}

fn produce_line(source: (i32, i32), path_instruction: Path) -> Vec<(i32, i32)> {
    use Path::*;
    let (direction_vector, distance) = match path_instruction {
        Up(d) => ((0, 1), d),
        Right(d) => ((1, 0), d),
        Down(d) => ((0, -1), d),
        Left(d) => ((-1, 0), d),
    };

    let mut cursor_position = source;
    let mut line = vec![];
    for _ in 0..distance {
        let (x1, y1) = cursor_position;
        let next_step = (x1 + direction_vector.0, y1 + direction_vector.1);
        line.push(next_step);
        cursor_position = next_step;
    }
    line
}

/// From a list of Path instructions, produces a list of coordinates
/// that denote all the points travelled if you follow the Path instructions
pub fn wire_to_trail(path: Vec<Path>) -> Vec<(i32, i32)> {
    // wires always start at origin
    let mut trail = vec![(0, 0)];
    for p in path {
        let source = trail[trail.len() - 1];
        let line = produce_line(source, p);
        trail.extend(line);
    }
    trail
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use Path::*;
        let line = "R2,U5";
        assert_eq!("R", line.get(0..1).unwrap());
        assert_eq!("2,U5", line.get(1..).unwrap());

        assert_eq!(vec![Right(2), Up(5)], to_wire(line));
    }

    #[test]
    fn test_generate_trail() {
        let line = "R2,U5";

        assert_eq!(
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5)
            ],
            wire_to_trail(to_wire(line))
        );
    }
}
