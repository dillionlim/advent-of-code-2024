// Implementation for Day 6, Part B
use crate::common::{get_input, process_input};
use std::path::Path;

const DIRS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

fn hash_coords(x: isize, y: isize, width: usize) -> usize {
    (x * width as isize + y) as usize
}

fn is_loop(grid: &Vec<Vec<char>>, startx: isize, starty: isize, seen: &mut Vec<u8>) -> bool {
    seen.fill(0);

    let mut direction_ind = 0;
    let mut x = startx;
    let mut y = starty;

    while in_bounds(x, y, grid.len(), grid[0].len()) {
        let newx = x + DIRS[direction_ind].0;
        let newy = y + DIRS[direction_ind].1;

        if !in_bounds(newx, newy, grid.len(), grid[0].len()) {
            return false;
        }

        if grid[newx as usize][newy as usize] == '#' {
            direction_ind = (direction_ind + 1) % DIRS.len();

            let masked_dir = 1u8 << direction_ind;

            if seen[hash_coords(x, y, grid[0].len())] & masked_dir > 0 {
                return true;
            }

            seen[hash_coords(x, y, grid[0].len())] |= masked_dir;
        } 
        else {
            x = newx;
            y = newy;
        }
    }
    false
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut grid: Vec<Vec<char>> = process_input(input, "char");

    let (startx, starty): (isize, isize) = grid.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &v)| if v == '^' { Some((i as isize, j as isize)) } else { None })
        })
        .unwrap_or((-1, -1));

    if startx == -1 || starty == -1 {
        return "0".to_string();
    }

    let mut direction_ind: usize = 0;
    let mut count: u32 = 0;
    let mut x = startx;
    let mut y = starty;

    let mut vis: Vec<bool> = vec![false; grid.len() * grid[0].len()];
    let mut seen: Vec<u8> = vec![0; grid.len() * grid[0].len()];

    while in_bounds(x, y, grid.len(), grid[0].len()) {
        let newx = x + DIRS[direction_ind].0;
        let newy = y + DIRS[direction_ind].1;

        if !in_bounds(newx, newy, grid.len(), grid[0].len()) {
            break;
        }

        if grid[newx as usize][newy as usize] == '#' {
            direction_ind = (direction_ind + 1) % DIRS.len();
        } 
        else {
            grid[newx as usize][newy as usize] = '#';
            if !vis[hash_coords(newx, newy, grid[0].len())] && is_loop(&grid, startx, starty, &mut seen) {
                count += 1;
                vis[hash_coords(newx, newy, grid[0].len())] = true;
            }
            grid[newx as usize][newy as usize] = '.';
            x = newx;
            y = newy;
        }
    }

    count.to_string()
}