// Implementation for Day 20, Part B
use crate::common::{get_input, process_input};
use std::path::Path;

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

    vis[currx as usize * grid[0].len() + curry as usize] = true;

    loop {
        path.push((currx, curry));
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
    }

    let mut ans = 0;

    for (i, (x, y)) in path.iter().enumerate() {
        let mut j = i + 100;
        while j < path.len() {
            let dist = (x - path[j].0).abs() + (y - path[j].1).abs();
            if dist <= 20 && j.abs_diff(i) >= dist as usize + 100 {
                ans += 1;
            }
            else if dist > 20 {
                j += dist as usize - 20;
                continue;
            }
            j += 1;
        }
    }
    
    return ans.to_string();
}
