// Implementation for Day 4, Part A
use crate::common::{get_input, process_input};
use std::path::Path;

const DIRS: &[(isize, isize)] = &[(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

fn get(grid: &Vec<Vec<char>>, x: isize, y: isize) -> Option<char> {
    if x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len() {
        Some(grid[x as usize][y as usize])
    } else {
        None
    }
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<Vec<char>> = process_input(input, "char");
    
    let answer: usize = processed_input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(move |(_, &v)| v == 'X')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .map(|(x, y)| {
            DIRS.iter()
                .filter(|&&(dx, dy)| {
                    (1..)
                        .zip("MAS".chars())
                        .all(|(c, target)| {
                            get(&processed_input, x + dx * c as isize, y + dy * c as isize) == Some(target)
                        })
                })
                .count()
        })
        .sum();
    
    return answer.to_string();
}
