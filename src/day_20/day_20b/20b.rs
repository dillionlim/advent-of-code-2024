// Implementation for Day 20, Part B
use crate::common::{get_input, process_input};
use std::path::Path;
use itertools::Itertools;

const DIRS: &[(isize, isize)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

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
    let mut currx = start.0 as isize;
    let mut curry = start.1 as isize;
    let mut ind = 0usize;

    vis[currx as usize * grid[0].len() + curry as usize] = true;

    loop {
        path.push((ind, currx, curry));
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

    let mut ans = 0;

    for ((i1, x1, y1), (i2, x2, y2)) in path.iter().tuple_combinations() {
        let d = x1.abs_diff(*x2) + y1.abs_diff(*y2);
        if d <= 20 && i1.abs_diff(*i2) >= d + 100 {
            ans += 1;
        }
    }
    
    return ans.to_string();
}
