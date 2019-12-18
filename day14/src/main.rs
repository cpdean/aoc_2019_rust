use std::error;
use std::fs;

#[derive(Debug, Clone)]
struct ParseError;
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug)]
struct Ingredient {
    quantity: i32,
    identifier: String,
}

#[derive(Debug)]
struct ProductionRule {
    input: Vec<Ingredient>,
    output: Ingredient,
}

impl std::str::FromStr for ProductionRule {
    type Err = ParseError;
    fn from_str(rule_line: &str) -> Result<ProductionRule, Self::Err> {
        let parts: Vec<_> = rule_line.split(" => ").collect();
        let input = parts[0].split(", ").map(|e| e.parse().unwrap()).collect();
        let output = parts[1].parse().unwrap();
        Ok(ProductionRule {
            input: input,
            output: output,
        })
    }
}

impl std::str::FromStr for Ingredient {
    type Err = ParseError;
    fn from_str(rule_line: &str) -> Result<Ingredient, Self::Err> {
        let parts: Vec<_> = rule_line.split(" ").collect();
        Ok(Ingredient {
            quantity: parts[0].parse().unwrap(),
            identifier: parts[1].to_string(),
        })
    }
}

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day14.txt")?.trim().to_string();
    let rules: Vec<ProductionRule> = f.split("\n").map(|e| e.parse().unwrap()).collect();
    for rule in rules {
        dbg!(rule);
    }

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
