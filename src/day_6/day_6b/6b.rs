// Implementation for Day 6, Part B
use crate::common::{get_input, process_input};
use std::path::Path;
use std::cmp::{max, min};

const DIRS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

fn create_jump_map(grid: &Vec<Vec<char>>) -> Vec<Vec<[isize; 4]>> {
    let mut jump_map: Vec<Vec<[isize; 4]>> = vec![vec![[0; 4]; grid[0].len()]; grid.len()];

    // jump map for the up direction
    for (col_idx, _) in grid[0].iter().enumerate() {
        let mut jump_map_ind = -1 as isize;
        for (row_idx, row) in grid.iter().enumerate() {
            if row[col_idx] == '#' {
                jump_map_ind = row_idx as isize + 1;
            } 
            else {
                jump_map[row_idx][col_idx][0] = jump_map_ind;
            }
        }
    }

    // jump map for the right direction
    for (row_idx, row) in grid.iter().enumerate() {
        let mut jump_map_ind = row.len() as isize;
        for (col_idx, &char) in row.iter().enumerate().rev() {
            if char == '#' {
                if col_idx > 0 {
                    jump_map_ind = col_idx as isize - 1;
                }
            }
            else {
                jump_map[row_idx][col_idx][1] = jump_map_ind;
            }
        }
    }

    // jump map for the down direction
    for (col_idx, _) in grid[0].iter().enumerate() {
        let mut jump_map_ind = grid[0].len() as isize;
        for (row_idx, row) in grid.iter().enumerate().rev() {
            if row[col_idx] == '#' {
                if row_idx > 0 {
                    jump_map_ind = row_idx as isize - 1;
                }
            } 
            else {
                jump_map[row_idx][col_idx][2] = jump_map_ind;
            }
        }
    }

    // jump map for the left direction
    for (row_idx, row) in grid.iter().enumerate() {
        let mut jump_map_ind = -1 as isize;
        for (col_idx, &char) in row.iter().enumerate() {
            if char == '#' {
                jump_map_ind = col_idx as isize + 1;
            }
            else {
                jump_map[row_idx][col_idx][3] = jump_map_ind;
            }
        }
    }

    jump_map
}

fn get_jump(jump_map: &Vec<Vec<[isize; 4]>>, x: isize, y: isize, obs_x: isize, obs_y: isize, dir: usize) -> (isize, isize) {
    match dir {
        0 => {
            if obs_y == y && x > obs_x {
                (max(obs_x + 1, jump_map[x as usize][y as usize][0]), y)
            }
            else {
                (jump_map[x as usize][y as usize][0], y)
            }
        },
        1 => {
            if obs_x == x && y < obs_y {
                (x, min(obs_y - 1, jump_map[x as usize][y as usize][1]))
            }
            else {
                (x, jump_map[x as usize][y as usize][1])
            }
        },
        2 => {
            if obs_y == y && x < obs_x {
                (min(obs_x - 1, jump_map[x as usize][y as usize][2]), y)
            }
            else {
                (jump_map[x as usize][y as usize][2], y)
            }
        },
        3 => {
            if obs_x == x && y > obs_y {
                (x, max(obs_y + 1, jump_map[x as usize][y as usize][3]))
            }
            else {
                (x, jump_map[x as usize][y as usize][3])
            }
        },
        _ => panic!("Invalid direction"),
    }
}

fn is_loop(jump_map: &Vec<Vec<[isize; 4]>>, startx: isize, starty: isize, obs_x: isize, obs_y: isize) -> bool {
    let mut vis = vec![0u8; jump_map.len() * jump_map[0].len()];

    let mut x = startx;
    let mut y = starty;
    let mut dir = 0;

    loop {
        (x, y) = get_jump(jump_map, x, y, obs_x, obs_y, dir);

        if !in_bounds(x, y, jump_map.len(), jump_map[0].len()) {
            break;
        }

        dir = (dir + 1) % DIRS.len();

        let idx = x as usize * jump_map[0].len() + y as usize;
        let masked_dir = 1u8 << dir;
        if vis[idx] & masked_dir > 0 {
            return true;
        }
        vis[idx] |= masked_dir;
    }
    false
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let grid: Vec<Vec<char>> = process_input(input, "char");

    let (startx, starty): (isize, isize) = grid.iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, &v)| if v == '^' { Some((i as isize, j as isize)) } else { None })
        })
        .unwrap();

    let jump_map = create_jump_map(&grid);

    let mut vis = vec![false; grid.len() * grid[0].len()];
    let mut x = startx;
    let mut y = starty;
    let mut dir_ind = 0;
    let mut ans = 0;
    
    loop {
        let (dx, dy) = DIRS[dir_ind];
        x += dx;
        y += dy;
        if !in_bounds(x, y, grid.len(), grid[0].len()) {
            break;
        }
        if grid[x as usize][y as usize] == '#' {
            x -= dx;
            y -= dy;
            dir_ind = (dir_ind + 1) % DIRS.len();
        }
        else if (x != startx || y != starty) && !vis[x as usize * grid[0].len() + y as usize] {
            vis[x as usize * grid[0].len() + y as usize] = true;
            if is_loop(&jump_map, startx, starty, x, y) {
                ans += 1;
            }
        }
    }
    
    ans.to_string()
}