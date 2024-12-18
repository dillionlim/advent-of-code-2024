// Implementation for Day 18, Part A
use crate::common::get_input;
use std::path::Path;
use std::collections::VecDeque;

const DIRS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn process_input(input: Vec<String>) -> Vec<(usize, usize)> {
    input
    .into_iter()
    .map(|line| {
        let split_vals = line
        .split_once(',')
        .unwrap();
        
        (split_vals.0.parse::<usize>().unwrap(), split_vals.1.parse::<usize>().unwrap())
    })
    .collect()
}

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

fn bfs(fallen: &mut Vec<bool>) -> usize {
    let mut queue = VecDeque::new();
    let mut vis = vec![false; 71 * 71];
    queue.push_back((0usize, 0usize));
    vis[0] = true;
    let mut ans = 0;

    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let (x, y) = queue.pop_front().unwrap();
            if x == 70 && y == 70 {
                return ans;
            }
            for (dx, dy) in DIRS {
                let newx = x as isize + dx;
                let newy = y as isize + dy;
                if in_bounds(newx, newy, 71, 71) 
                && !fallen[newx as usize * 71 + newy as usize] 
                && !vis[newx as usize * 71 + newy as usize] {
                    queue.push_back((newx as usize, newy as usize));
                    vis[newx as usize * 71 + newy as usize] = true;
                }
            }
        }
        ans += 1;
    }
    0
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<(usize, usize)> = process_input(input);

    let mut fallen = vec![false; 71 * 71];
    for i in 0..12 {
        fallen[processed_input[i].1 as usize * 71 + processed_input[i].0 as usize] = true;
    }
    
    return bfs(&mut fallen).to_string();
}
