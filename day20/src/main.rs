use std::collections::{HashMap, HashSet};
use std::fs;

pub fn main() -> std::io::Result<()> {
    let _f = fs::read_to_string("input/day20.txt")?.trim().to_string();
    dbg!("goat");

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum CellType {
    Wall,
    Path,
    Portal((char, char)),
}

#[derive(Debug, PartialEq)]
pub struct Cell {
    cell_type: CellType,
    x: usize,
    y: usize,
}

pub enum Dir {
    Up,
    Right,
    Down,
    Left,
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

pub fn portal_label(grid: &Vec<Vec<char>>, x: usize, y: usize, direction: Dir) -> (char, char) {
    match direction {
        Dir::Up => (grid[y - 2][x], grid[y - 1][x]),
        Dir::Right => (grid[y][x + 1], grid[y][x + 2]),
        Dir::Down => (grid[y + 1][x], grid[y + 2][x]),
        Dir::Left => (grid[y][x - 2], grid[y][x - 1]),
    }
}

pub fn parse_donut_map(s: String, donut_thickness: usize) -> HashMap<(usize, usize), CellType> {
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
            field.insert((x, y), c);
        }
    }
    // relabel portals

    // scan top
    let outer_top_edge = 2 as usize;
    for x in 2..(grid[0].len()) - 2 {
        if let Some(&CellType::Path) = field.get(&(x, outer_top_edge)) {
            let label = portal_label(&grid, x, outer_top_edge, Dir::Up);
            field.insert((x, outer_top_edge), CellType::Portal(label));
        }
    }

    // scan inner top
    let inner_top_edge = 2 as usize + donut_thickness - 1;
    for x in 2 + donut_thickness..(grid[0].len()) - 2 - donut_thickness {
        if let Some(&CellType::Path) = field.get(&(x, inner_top_edge)) {
            let label = portal_label(&grid, x, inner_top_edge, Dir::Down);
            field.insert((x, inner_top_edge), CellType::Portal(label));
        }
    }

    // scan outer left
    let outer_left_edge = 2 as usize;
    for y in 2..(grid.len()) + donut_thickness - 2 {
        if let Some(&CellType::Path) = field.get(&(outer_left_edge, y)) {
            let label = portal_label(&grid, outer_left_edge, y, Dir::Left);
            field.insert((outer_left_edge, y), CellType::Portal(label));
        }
    }

    // scan inner left
    let inner_left_edge = 2 as usize + donut_thickness - 1;
    for y in (2 + donut_thickness)..((grid.len()) - donut_thickness - 2) {
        if let Some(&CellType::Path) = field.get(&(inner_left_edge, y)) {
            let label = portal_label(&grid, inner_left_edge, y, Dir::Right);
            field.insert((inner_left_edge, y), CellType::Portal(label));
        }
    }

    // scan inner right
    let inner_right_edge = grid[0].len() - 2 - donut_thickness;
    for y in (2 + donut_thickness)..((grid.len()) - donut_thickness - 2) {
        if let Some(&CellType::Path) = field.get(&(inner_right_edge, y)) {
            let label = portal_label(&grid, inner_right_edge, y, Dir::Left);
            field.insert((inner_right_edge, y), CellType::Portal(label));
        }
    }

    // scan outer right
    let outer_right_edge = grid[0].len() - 2 - 1;
    for y in (2)..((grid.len()) - 2) {
        if let Some(&CellType::Path) = field.get(&(outer_right_edge, y)) {
            let label = portal_label(&grid, outer_right_edge, y, Dir::Right);
            field.insert((outer_right_edge, y), CellType::Portal(label));
        }
    }

    // scan outer bottom
    let outer_bottom_edge = grid.len() - 2 - 1;
    for x in (2)..((grid[0].len()) - 2) {
        if let Some(&CellType::Path) = field.get(&(x, outer_bottom_edge)) {
            let label = portal_label(&grid, x, outer_bottom_edge, Dir::Down);
            field.insert((x, outer_bottom_edge), CellType::Portal(label));
        }
    }

    // scan inner bottom
    let inner_bottom_edge = grid.len() - 2 - donut_thickness;
    for x in (2 + donut_thickness)..((grid[0].len()) - 2 - donut_thickness) {
        if let Some(&CellType::Path) = field.get(&(x, inner_bottom_edge)) {
            let label = portal_label(&grid, x, inner_bottom_edge, Dir::Up);
            field.insert((x, inner_bottom_edge), CellType::Portal(label));
        }
    }

    field
}

pub fn edge_map(
    field: &HashMap<(usize, usize), CellType>,
) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
    let mut paths = HashMap::new();
    for (k, v) in field {
        match v {
            CellType::Path | CellType::Portal(_) => paths.insert(k, v),
            CellType::Wall => continue,
        };
    }

    let mut adjacency = HashMap::new();
    for ((x, y), _) in &paths {
        let mut edges = HashSet::new();
        let neighbors: Vec<(usize, usize)> =
            vec![(x + 1, *y), (x - 1, *y), (*x, y + 1), (*x, y - 1)];
        for n in neighbors {
            if let Some(_) = paths.get(&n) {
                edges.insert(n);
            }
        }
        adjacency.insert((*x, *y), edges);
    }
    adjacency
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiny_map_raw() -> String {
        "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######..GG
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##     GG.##.#  
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

    #[test]
    fn test_parse_tiny_portal_labels() {
        let p = parse_donut_map(tiny_map_raw(), 5);
        // the outer AA portal
        assert_eq!(p.get(&(9, 2)), Some(&CellType::Portal(('A', 'A'))));
        // the inner BC portal
        assert_eq!(p.get(&(9, 6)), Some(&CellType::Portal(('B', 'C'))));
        // the outer DE portal
        assert_eq!(p.get(&(2, 13)), Some(&CellType::Portal(('D', 'E'))));
        // the inner DE portal
        assert_eq!(p.get(&(6, 10)), Some(&CellType::Portal(('D', 'E'))));
        // the inner GG portal
        assert_eq!(p.get(&(14, 9)), Some(&CellType::Portal(('G', 'G'))));
        // the outer GG portal
        assert_eq!(p.get(&(18, 4)), Some(&CellType::Portal(('G', 'G'))));
        // the outer ZZ portal
        assert_eq!(p.get(&(13, 16)), Some(&CellType::Portal(('Z', 'Z'))));
        // the inner FG portal
        assert_eq!(p.get(&(11, 12)), Some(&CellType::Portal(('F', 'G'))));
    }

    #[test]
    fn test_parse_tiny() {
        let p = parse_donut_map(tiny_map_raw(), 5);
        assert_eq!(p.get(&(2, 2)), Some(&CellType::Wall));
        assert_eq!(p.get(&(6, 6)), Some(&CellType::Wall));
        // first empty corner
        assert_eq!(p.get(&(7, 7)), None);
        // last empty along the inner top
        assert_eq!(p.get(&(13, 7)), None);
        assert_eq!(p.get(&(14, 7)), Some(&CellType::Wall));
        // last empty bottom inner right
        assert_eq!(p.get(&(13, 11)), None);
        // inner corner should have something
        assert_eq!(p.get(&(14, 12)), Some(&CellType::Wall));
    }

    /// path cells adjacent to each other should have edges
    #[test]
    fn edge_map_finds_basic_connections() {
        let p = parse_donut_map(tiny_map_raw(), 5);
        let edges = edge_map(&p);
        let tee_intersection = edges.get(&(9, 3)).unwrap();
        assert_eq!(tee_intersection.len(), 3);
        let hallway = edges.get(&(9, 4)).unwrap();
        assert_eq!(hallway.len(), 2);
    }
}
