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
    let input_state: Vec<&str> = f.trim().split("\n").collect();
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

/// From a list of Path instructions, produces a list of coordinates
/// that denote all the points travelled if you follow the Path instructions
pub fn wire_to_trail(path: Vec<Path>) -> Vec<(i32, i32)> {
    // wires always start at origin
    let trail = vec![(0, 0)];
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
}
