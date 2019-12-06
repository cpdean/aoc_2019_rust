use std::collections::HashMap;
use std::fs;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day06.txt")?;
    let map_pairs: Vec<(&str, &str)> = f
        .trim()
        .split("\n")
        .map(|line| {
            let divided = line.split(")").collect::<Vec<&str>>();
            let (root, leaf) = (divided[0], divided[1]);
            (root, leaf)
        })
        .collect();
    let mut orbital_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (root, leaf) in map_pairs {
        if orbital_map.contains_key(root) {
            let child_list: &Vec<&str> = orbital_map.get(root).unwrap();
            child_list.push(leaf);
        } else {
            orbital_map.insert(root, vec![leaf]).unwrap();
        }
    }
    // root -> children
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        /*
                let uh = """
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        """;
        */
    }
}
