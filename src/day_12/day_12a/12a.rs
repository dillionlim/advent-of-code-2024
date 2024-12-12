// Implementation for Day 12, Part A
use crate::common::{get_input, process_input};
use std::path::Path;
use std::collections::VecDeque;

const DIRS: &[(isize, isize)] = &[(-1,0),(0,-1),(0,1),(1,0)];

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<Vec<char>> = process_input(input, "char");

    let mut vis = vec![false; processed_input.len() * processed_input[0].len()];
    let mut ans = 0;

    for (i, row) in processed_input.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            let mut perimeter = 0;
            let mut area = 0;
            if !vis[i * processed_input[0].len() + j] {
                let mut queue = VecDeque::new();
                queue.push_back((i as isize, j as isize));
                vis[i * processed_input[0].len() + j] = true;

                while let Some((x, y)) = queue.pop_front() {
                    area += 1;
                    for (dx, dy) in DIRS {
                        let new_x = x + dx;
                        let new_y = y + dy;

                        if in_bounds(new_x, new_y, processed_input.len(), processed_input[0].len()) 
                        && processed_input[new_x as usize][new_y as usize] == ch {
                            if !vis[new_x as usize * processed_input[0].len() + new_y as usize] {
                                vis[new_x as usize * processed_input[0].len() + new_y as usize] = true;
                                queue.push_back((new_x, new_y));
                            }
                        }
                        else {
                            perimeter += 1;
                        }
                    }
                }
            }
            ans += area * perimeter;
        }
    }
    
    return ans.to_string();
}
