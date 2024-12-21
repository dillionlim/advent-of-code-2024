// Implementation for Day 20, Part A
use crate::common::{get_input, process_input};
use std::path::Path;
use hashbrown::HashSet;

const DIRS: &[(isize, isize)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let grid: Vec<Vec<char>> = process_input(input, "char");

    let start = grid
    .iter()
    .enumerate()
    .filter_map(|(i, row)| {
        row.iter()
            .enumerate()
            .find_map(|(j, &v)| if v == 'S' { Some((i as isize, j as isize)) } else { None })
    })
    .next()
    .unwrap();

    let mut vis = vec![false; grid.len() * grid[0].len()];
    let mut path = Vec::new();
    let mut on_path = vec![0usize; grid.len() * grid[0].len()];
    let mut currx = start.0 as isize;
    let mut curry = start.1 as isize;
    let mut ind = 0;

    vis[currx as usize * grid[0].len() + curry as usize] = true;

    loop {
        path.push((ind, currx, curry));
        on_path[currx as usize * grid[0].len() + curry as usize] = ind;
        if grid[currx as usize][curry as usize] == 'E' {
            break;
        }
        for (dx, dy) in DIRS {
            let newx = currx as isize + dx;
            let newy = curry as isize + dy;
            if (grid[newx as usize][newy as usize] == '.' ||
            grid[newx as usize][newy as usize] == 'E')
            && !vis[newx as usize * grid[0].len() + newy as usize] {
                vis[newx as usize * grid[0].len() + newy as usize] = true;
                currx = newx;
                curry = newy;
                break;
            }
        }
        ind += 1;
    }
    
    const MAXDIST: isize = 2;
    let mut ans = HashSet::new();

    for (i, x, y) in path.iter() {
        for dx in -MAXDIST..=MAXDIST {
            let max_dy = MAXDIST - dx.abs();
            for dy in -max_dy..=max_dy {
                let newx = x + dx;
                let newy = y + dy;
                let dist = newx.abs_diff(*x) + newy.abs_diff(*y);

                if in_bounds(newx, newy, grid.len(), grid[0].len()) 
                && on_path[newx as usize * grid[0].len() + newy as usize] != 0
                && on_path[newx as usize * grid[0].len() + newy as usize].abs_diff(*i) >= dist + 100 {
                    let new_hash = newx as usize * grid[0].len() + newy as usize;
                    let old_hash = *x as usize * grid[0].len() + *y as usize;
                    if new_hash > old_hash {
                        ans.insert((new_hash, old_hash));
                    }
                    else {
                        ans.insert((old_hash, new_hash));
                    }
                }
            }
        }    
    }
    
    return ans.len().to_string();
}
