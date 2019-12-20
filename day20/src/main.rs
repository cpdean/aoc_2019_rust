use std::fs;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day20.txt")?.trim().to_string();
    //dbg!(part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiny() {
        assert_eq!(1, 2);
    }
}
