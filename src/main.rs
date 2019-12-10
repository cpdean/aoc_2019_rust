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

fn slope(a: (f64, f64), b: (f64, f64)) -> f64 {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;
    (b_y - a_y) / (b_x - a_x)
}

pub fn points_are_on_a_line(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
    let ab = slope(a, b);
    let ac = slope(a, c);
    let bc = slope(b, c);
    ab == ac && ac == bc
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
        assert_eq!(positions.len(), 10)
    }

    #[test]
    fn line_test1() {
        let a = (0.0, 0.0);
        let b = (0.0, 1.0);
        let c = (1.0, 0.0);
        assert_eq!(points_are_on_a_line(a, b, c), false);
    }

    #[test]
    fn line_test2() {
        let a = (0.0, 0.0);
        let b = (1.0, 1.0);
        let c = (1.0, 3.0);
        assert_eq!(points_are_on_a_line(a, b, c), false);
    }

    #[test]
    fn line_test3() {
        let a = (0.0, 0.0);
        let b = (0.0, 1.0);
        let c = (0.0, 3.0);
        assert_eq!(points_are_on_a_line(a, b, c), true);
    }

    #[test]
    fn line_test4() {
        let a = (2.0, 4.0);
        let b = (4.0, 6.0);
        let c = (6.0, 8.0);
        assert_eq!(points_are_on_a_line(a, b, c), true);
    }
}
