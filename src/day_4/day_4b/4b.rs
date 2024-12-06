// Implementation for Day 4, Part A
use crate::common::{get_input, process_input};
use std::path::Path;

// place the position of both "M"s, because the position of the corresponding "S" is just -1 * (x, y).
const DIRS: &[((isize, isize), (isize, isize))] = &[((-1,-1),(1,-1)),((-1,-1),(-1,1)),((-1,1),(1,1)),((1,-1),(1,1))];

fn get(grid: &Vec<Vec<char>>, x: isize, y: isize) -> Option<char> {
    if x >= 0 && y >= 0 && (x as usize) < grid.len() && (y as usize) < grid[0].len() {
        Some(grid[x as usize][y as usize])
    } else {
        None
    }
}

fn check(grid: &Vec<Vec<char>>, firstm: (isize, isize), secondm: (isize, isize), x: isize, y: isize) -> bool {
    let first_m_pos = (x + firstm.0, y + firstm.1);
    let first_s_pos = (x - firstm.0, y - firstm.1);
    let second_m_pos = (x + secondm.0, y + secondm.1);
    let second_s_pos = (x - secondm.0, y - secondm.1);

    match (
        get(grid, first_m_pos.0, first_m_pos.1),
        get(grid, first_s_pos.0, first_s_pos.1),
        get(grid, second_m_pos.0, second_m_pos.1),
        get(grid, second_s_pos.0, second_s_pos.1),
    ) {
        (Some('M'), Some('S'), Some('M'), Some('S')) => true,
        _ => false,
    }
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<Vec<char>> = process_input(input, "char");
    let input_ref = &processed_input;
    
    let answer: usize = processed_input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(move |(_, &v)| v == 'A')
                .map(move |(j, _)| (i as isize, j as isize))
        })
        .map(|(x, y)| {
            DIRS.iter()
                .filter(move |&&(firstm, secondm)| check(input_ref, firstm, secondm, x, y))
                .count()
        })
        .sum();
    
    return answer.to_string();
}
