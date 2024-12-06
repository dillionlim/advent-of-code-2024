// Implementation for Day 6, Part B
use crate::common::{get_input, process_input};
use std::path::Path;
use std::collections::HashSet;

const DIRS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

fn is_loop(grid: &Vec<Vec<char>>, startx: isize, starty: isize) -> bool {
    let mut seen: HashSet<(isize, isize, usize)> = HashSet::new();
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
            if seen.contains(&(newx, newy, direction_ind)) {
                return true;
            }
            seen.insert((newx, newy, direction_ind));
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

    let mut vis: HashSet<(isize, isize)> = HashSet::new();

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
            if !vis.contains(&(newx, newy)) && is_loop(&grid, startx, starty) {
                count += 1;
                vis.insert((newx, newy));
            }
            grid[newx as usize][newy as usize] = '.';
            x = newx;
            y = newy;
        }
    }

    count.to_string()
}