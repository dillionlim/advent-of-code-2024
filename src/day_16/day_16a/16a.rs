// Implementation for Day 16, Part A
use crate::common::{get_input, process_input};
use std::path::Path;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

const DIRS: &[(u8, isize, isize)] = &[(0, 0, 1), (1, -1, 0), (2, 0, -1), (3, 1, 0)];

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let grid: Vec<Vec<char>> = process_input(input, "char");

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in grid.iter().enumerate() {
        for (j, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                start = (i, j);
            }
            else if ch == 'E' {
                end = (i, j);
            }
        }
    }

    let mut pq: BinaryHeap<(Reverse<u32>, u8, usize, usize)> = BinaryHeap::new(); // (weight, dir, x, y)
    let mut distances = vec![0x3f3f3f3f as u32; grid.len() * grid[0].len()];
    distances[start.0 * grid[0].len() + start.1] = 0;

    pq.push((Reverse(0), 0u8, start.0, start.1));

    while let Some((Reverse(dist), dir, x, y)) = pq.pop() {
        if dist != distances[x * grid[0].len() + y] {
            continue;
        }
        if (x, y) == end {
            break;
        }
        for (i, dx, dy) in DIRS {
            let newx = (x as isize + dx) as usize;
            let newy = (y as isize + dy) as usize;

            if grid[newx][newy] != '#' {
                let no_of_rotations = (*i as i8 - dir as i8).abs() as u8;
                let dist = if no_of_rotations == 0 { 1 } else if no_of_rotations == 1 || no_of_rotations ==3 { 1001 } else { 2001 };
                if distances[x * grid[0].len() + y] + dist < 
                distances[newx * grid[0].len() + newy] {
                    distances[newx * grid[0].len() + newy] = distances[x * grid[0].len() + y] + dist;
                    pq.push((Reverse(distances[newx * grid[0].len() + newy]), *i, newx, newy));
                }
            }
        }
    }
    
    return distances[end.0 * grid[0].len() + end.1].to_string();
}
