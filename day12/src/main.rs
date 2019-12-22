use std::fs;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day12.txt")?.trim().to_string();
    let mut bodies = parse_bodies(f);

    for step in 1..=1001 {
        bodies = apply_gravity(&bodies);
        bodies = apply_velocity(bodies);
        let e = energy(&bodies);
        dbg!((step, e));
    }

    Ok(())
}

fn parse_bodies(s: String) -> Vec<Body> {
    let mut bodies = vec![];
    for body in s.split("\n") {
        let coord_hunks: Vec<&str> = body.split(", ").collect();
        let x_hunk = coord_hunks[0];
        let y_hunk = coord_hunks[1];
        let z_hunk = coord_hunks[2];

        // x_hunk: "<x=14"
        let x = x_hunk.split("=").last().unwrap().parse::<i32>();
        // y_hunk: "y=11"
        let y = y_hunk.split("=").last().unwrap().parse::<i32>();
        // z_hunk: "z=14>"
        let z = z_hunk[..(z_hunk.len() - 1)]
            .split("=")
            .last()
            .unwrap()
            .parse::<i32>();
        let b = Point {
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),
        };
        bodies.push(Body {
            pos: b,
            vel: Point::new(),
        });
    }
    bodies
}

fn difference(a: &i32, b: &i32) -> i32 {
    if a < b {
        1
    } else if a > b {
        -1
    } else if a == b {
        0
    } else {
        panic!("how are these not comparable {} {}", a, b);
    }
}

fn accelleration_changes(a: &Point, b: &Point) -> (Point, Point) {
    let Point {
        x: a_x,
        y: a_y,
        z: a_z,
        ..
    } = a;
    let Point {
        x: b_x,
        y: b_y,
        z: b_z,
        ..
    } = b;

    let a_accelleration = Point {
        x: difference(a_x, b_x),
        y: difference(a_y, b_y),
        z: difference(a_z, b_z),
    };

    let b_accelleration = a_accelleration.invert();
    (a_accelleration, b_accelleration)
}

pub fn apply_gravity(bodies: &Vec<Body>) -> Vec<Body> {
    let mut out_bodies = vec![];
    for i in 0..bodies.len() {
        let this_body = bodies[i];
        let mut changed_body = this_body.clone();
        for j in 0..bodies.len() {
            let (this_accel, _) = accelleration_changes(&this_body.pos, &bodies[j].pos);
            changed_body.vel = changed_body.vel.add(&this_accel);
        }
        out_bodies.push(changed_body);
    }
    out_bodies
}

pub fn apply_velocity(mut bodies: Vec<Body>) -> Vec<Body> {
    for b in &mut bodies {
        b.pos = b.pos.add(&b.vel);
    }
    bodies
}

pub fn energy(bodies: &Vec<Body>) -> i32 {
    bodies
        .iter()
        .map(|b| {
            let potential = b.pos.x.abs() + b.pos.y.abs() + b.pos.z.abs();
            let kinetic = b.vel.x.abs() + b.vel.y.abs() + b.vel.z.abs();
            potential * kinetic
        })
        .fold(0, |a, b| a + b)
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0, z: 0 }
    }

    fn invert(&self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Body {
    pos: Point,
    vel: Point,
}

#[allow(unused_variables, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    fn tiny_example() -> Vec<Body> {
        vec![
            Body {
                pos: Point { x: -1, y: 0, z: 2 },
                vel: Point::new(),
            },
            Body {
                pos: Point {
                    x: 2,
                    y: -10,
                    z: -7,
                },
                vel: Point::new(),
            },
            Body {
                pos: Point { x: 4, y: -8, z: 8 },
                vel: Point::new(),
            },
            Body {
                pos: Point { x: 3, y: 5, z: -1 },
                vel: Point::new(),
            },
        ]
    }

    #[test]
    fn test_tiny_step1() {
        let mut bodies = tiny_example();
        for b in &bodies {
            assert_eq!(b.vel, Point::new());
        }
        bodies = apply_gravity(&bodies);
        for b in &bodies {
            if b.pos == (Point { x: -1, y: 0, z: 2 }) {
                assert_eq!(b.vel, (Point { x: 3, y: -1, z: -1 }));
            }
        }
        bodies = apply_velocity(bodies);
        for b in &bodies {
            if b.vel == (Point { x: 3, y: -1, z: -1 }) {
                assert_eq!(b.pos, (Point { x: 2, y: -1, z: 1 }));
            }
        }
    }

    #[test]
    fn test_tiny_step10() {
        let mut bodies = tiny_example();
        for b in &bodies {
            assert_eq!(b.vel, Point::new());
        }
        for _ in 0..10 {
            bodies = apply_gravity(&bodies);
            bodies = apply_velocity(bodies);
        }
        /*
        After 10 steps:
           pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
           pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
           pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
           pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>
        */
        for b in &bodies {
            if b.pos == (Point { x: 2, y: 1, z: -3 }) {
                assert_eq!(b.vel, (Point { x: -3, y: -2, z: 1 }));
            }
            if b.pos == (Point { x: 1, y: -8, z: 0 }) {
                assert_eq!(b.vel, (Point { x: -1, y: 1, z: 3 }));
            }
            if b.pos == (Point { x: 3, y: -6, z: 1 }) {
                assert_eq!(b.vel, (Point { x: 3, y: 2, z: -3 }));
            }
            if b.pos == (Point { x: 2, y: 0, z: 4 }) {
                assert_eq!(b.vel, (Point { x: 1, y: -1, z: -1 }));
            }
        }
    }
    #[test]
    fn test_tiny_energy_step10() {
        let mut bodies = tiny_example();
        for b in &bodies {
            assert_eq!(b.vel, Point::new());
        }
        for _ in 0..10 {
            bodies = apply_gravity(&bodies);
            bodies = apply_velocity(bodies);
        }
    }
}
