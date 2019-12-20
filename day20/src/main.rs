use std::collections::HashMap;
use std::fs;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day20.txt")?.trim().to_string();
    dbg!("goat");

    Ok(())
}

enum CellType {
    Wall,
    Path,
    Portal(char, char),
}

struct Cell {
    cell_type: CellType,
    x: usize,
    y: usize,
}

fn in_negative_space(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    donut_thickness: usize,
) -> bool {
    let past_left_edge = x >= 2 + donut_thickness;
    let past_right_edge = x < width - 2 - donut_thickness;
    let past_top_edge = y >= 2 + donut_thickness;
    let past_bottom_edge = y < height - 2 - donut_thickness;
    let in_middle_width = past_left_edge && past_right_edge;
    let in_middle_height = past_top_edge && past_bottom_edge;
    in_middle_width && in_middle_height
}

pub fn parse_donut_map(s: String, donut_thickness: usize) -> HashMap<(usize, usize), char> {
    // all donut inputs appear to have a 2 character margin around the edge
    // also, the width of a section appears to be consistent all around
    let grid: Vec<Vec<char>> = s.split("\n").map(|line| line.chars().collect()).collect();

    let mut field = HashMap::new();
    for y in 2..(grid.len()) - 2 {
        for x in 2..(grid[0].len()) - 2 {
            if in_negative_space(x, y, grid[0].len(), grid.len(), donut_thickness) {
                continue;
            }
            let c = match grid[y][x] {
                '.' => CellType::Path,
                '#' => CellType::Wall,
                e => panic!("unknown type: '{}', at {}.{}", e, x, y),
            };
            field.insert((x, y), grid[y][x]);
        }
    }
    field
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny_map_raw() -> String {
        "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "
            .to_string()
    }
    // 17 wide
    // 15 tall
    #[test]
    fn test_parse_tiny() {
        let p = parse_donut_map(tiny_map_raw(), 5);
        assert_eq!(*p.get(&(2, 2)).unwrap(), '#');
        assert_eq!(*p.get(&(6, 6)).unwrap(), '#');
        // first empty corner
        assert_eq!(p.get(&(7, 7)), None);
        // last empty along the inner top
        assert_eq!(p.get(&(13, 7)), None);
        assert_eq!(p.get(&(14, 7)), Some(&'#'));
        // last empty bottom inner right
        assert_eq!(p.get(&(13, 11)), None);
        // inner corner should have something
        assert_eq!(p.get(&(14, 12)), Some(&'#'));

        // the inner BC portal
        assert_eq!(p.get(&(9, 6)), Some(&'.'));
        // the outer BC portal
        assert_eq!(p.get(&(2, 8)), Some(&'.'));
    }
}
