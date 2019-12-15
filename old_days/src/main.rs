fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(11, 11);
    }
}
