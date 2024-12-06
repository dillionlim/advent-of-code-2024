// Implementation for Day 6, Part A
use crate::common::{get_input, process_input};
use std::path::Path;

const DIRS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(x: isize, y: isize, x_len: isize, y_len: isize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut processed_input: Vec<Vec<char>> = process_input(input, "char");
    
    let (mut x, mut y): (isize, isize) = processed_input.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
            .enumerate()
            .find_map(|(j, &v)| {
                if v == '^' { 
                    Some((i as isize, j as isize)) 
                }
                else { 
                    None 
                }
            })
        })
        .unwrap_or((-1, -1));

    let mut direction_ind: usize = 0;
    let mut count: u32 = 1;

    processed_input[x as usize][y as usize] = 'X';

    while in_bounds(x, y, processed_input.len() as isize, processed_input[0].len() as isize) {
        let newx = x + DIRS[direction_ind].0;
        let newy = y + DIRS[direction_ind].1;

        if in_bounds(newx, newy, processed_input.len() as isize, processed_input[0].len() as isize) && processed_input[newx as usize][newy as usize] == '#' {
            direction_ind = (direction_ind + 1) % DIRS.len();
        } 
        else if in_bounds(newx, newy, processed_input.len() as isize, processed_input[0].len() as isize) && processed_input[newx as usize][newy as usize] == '.' {
            count += 1;
            processed_input[newx as usize][newy as usize] = 'X';
            x = newx;
            y = newy;
        }
        else {
            x = newx;
            y = newy;
        }
    }

    return count.to_string();
}
