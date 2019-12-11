/// replace
use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day10.txt")?;
    let asteroid_positions = parse_asteroid_field(&f);
    dbg!(asteroid_positions);
    /*
    let layers = image_to_layers(input_state, 25, 6);
    let target_layer = layers
        .iter()
        .min_by(|a, b| {
            let a_zeros = a.chars().filter(|c| *c == '0').count();
            let b_zeros = b.chars().filter(|c| *c == '0').count();
            a_zeros.cmp(&b_zeros)
        })
        .unwrap();
    let ones = target_layer.chars().filter(|c| *c == '1').count();
    let twos = target_layer.chars().filter(|c| *c == '2').count();
    let part1 = ones * twos;
    dbg!(part1);
    let data = "0222112222120000";
    let layers_ex = image_to_layers(data, 2, 2);
    dbg!(layers_to_image(layers_ex.clone()));
    render(&layers_to_image(layers_ex), 2, 2);
    render(&layers_to_image(layers), 25, 6);
    */
    Ok(())
}

pub fn parse_asteroid_field(s: &str) -> Vec<(f64, f64)> {
    let lines = s.trim().split("\n");
    let mut asteroid_positions = vec![];
    for (y, line) in lines.enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                asteroid_positions.push((x as f64, y as f64));
            }
        }
    }
    asteroid_positions
}

type Point = (f64, f64);

fn slope(a: &Point, b: &Point) -> f64 {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;
    (b_y - a_y) / (b_x - a_x)
}

fn distance(a: &Point, b: &Point) -> f64 {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;
    ((b_x - a_x).exp2() + (b_y - a_y).exp2()).sqrt()
}

pub fn points_are_on_a_line(a: &Point, b: &Point, c: &Point) -> bool {
    let ab = slope(a, b);
    let ac = slope(a, c);
    let bc = slope(b, c);
    ab == ac && ac == bc
}

pub fn score_of_base(possible_base: &Point, mut asteroids: Vec<Point>) -> usize {
    asteroids.sort_by(|a, b| {
        let a_distance = distance(&possible_base, a);
        let b_distance = distance(&possible_base, b);
        a_distance.partial_cmp(&b_distance).unwrap()
    });
    let mut non_blocked_asteroids: Vec<_> = vec![];
    for p in asteroids {
        let mut p_is_blocked = false;
        for asteroid in &non_blocked_asteroids {
            if points_are_on_a_line(&possible_base, &p, asteroid) {
                p_is_blocked = true;
                break;
            }
        }
        if !p_is_blocked {
            non_blocked_asteroids.push(p);
        }
    }
    non_blocked_asteroids.len()
}

pub fn find_best_station(positions: Vec<Point>) -> (Point, usize) {
    let mut best_so_far = ((0.0, 0.0), 0);
    for i in 0..positions.len() {
        let possible_base = positions[i];
        let head = &positions[0..i];
        let tail = &positions[i + 1..];
        let mut ordered: Vec<Point> = [head, tail].concat();
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
        assert_eq!(best_point, ((3.0, 4.0), 8));
    }

    #[test]
    fn line_test1() {
        let a = (0.0, 0.0);
        let b = (0.0, 1.0);
        let c = (1.0, 0.0);
        assert_eq!(points_are_on_a_line(&a, &b, &c), false);
    }

    #[test]
    fn line_test2() {
        let a = (0.0, 0.0);
        let b = (1.0, 1.0);
        let c = (1.0, 3.0);
        assert_eq!(points_are_on_a_line(&a, &b, &c), false);
    }

    #[test]
    fn line_test3() {
        let a = (0.0, 0.0);
        let b = (0.0, 1.0);
        let c = (0.0, 3.0);
        assert_eq!(points_are_on_a_line(&a, &b, &c), true);
    }

    #[test]
    fn line_test4() {
        let a = (2.0, 4.0);
        let b = (4.0, 6.0);
        let c = (6.0, 8.0);
        assert_eq!(points_are_on_a_line(&a, &b, &c), true);
    }
}
