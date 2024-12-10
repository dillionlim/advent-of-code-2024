// Implementation for Day 10, Part A
use crate::common::get_input;
use std::path::Path;
use std::collections::VecDeque;

const DIRS: &[(isize, isize)] = &[(-1,0),(0,-1),(0,1),(1,0)];

pub fn process_input(input: Vec<String>) -> Vec<Vec<u8>> {
    input
        .into_iter()
        .map(|line| {
            line.chars()
                .filter_map(|c| {
                    let parsed = c.to_string().parse::<u8>();
                    parsed.ok()
                })
                .collect()
        })
        .collect()
}

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<Vec<u8>> = process_input(input);

    let answer: usize = processed_input
    .iter()
    .enumerate()
    .flat_map(|(i, row)| 
        row.iter()
           .enumerate()
           .filter(move |(_, &value)| value == 0)
           .map(move |(j, _)| (i as isize, j as isize))
    )
    .map(|(x, y)| {
        let mut queue: VecDeque<(isize, isize, u8)> = VecDeque::new();
        let mut vis = vec![0; processed_input.len() * processed_input[0].len()];
        queue.push_back((x, y, 0));
        vis[x as usize * processed_input[0].len() + y as usize] = 1;
        let mut rating = 0;
        while !queue.is_empty() {
            let (current_x, current_y, depth) = queue.pop_front().unwrap();
            for i in 0usize..4 {
                let new_x = current_x + DIRS[i].0;
                let new_y = current_y + DIRS[i].1;
                if in_bounds(new_x, new_y, processed_input.len(), processed_input[0].len()) 
                && processed_input[new_x as usize][new_y as usize] == depth + 1 {
                    if processed_input[new_x as usize][new_y as usize] == 9 {
                        rating += vis[current_x as usize * processed_input[0].len() + current_y as usize];
                    }
                    else if vis[new_x as usize * processed_input[0].len() + new_y as usize] == 0 {
                        queue.push_back((new_x, new_y, depth + 1));
                    }
                    vis[new_x as usize * processed_input[0].len() + new_y as usize] += 
                        vis[current_x as usize * processed_input[0].len() + current_y as usize];
                }
            }
        }
        rating 
    })
    .sum();
    
    return answer.to_string();
}
