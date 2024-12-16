// Implementation for Day 16, Part B
use crate::common::{get_input, process_input};
use std::path::Path;
use std::collections::{BinaryHeap, VecDeque, HashSet};
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
    let mut distances = vec![0x3f3f3f3f as u32; grid.len() * grid[0].len() * 4];
    distances[((start.0 * grid[0].len() + start.1) << 2) | 0] = 0;
    let mut parent: Vec<Vec<(usize, usize, u8)>> = vec![Vec::new(); grid.len() * grid[0].len() * 4];

    pq.push((Reverse(0), 0u8, start.0, start.1));

    while let Some((Reverse(dist), dir, x, y)) = pq.pop() {
        if dist != distances[((x * grid[0].len() + y) << 2) | dir as usize] {
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
                let newdist = if no_of_rotations == 0 { 1 } else if no_of_rotations == 1 || no_of_rotations == 3 { 1001 } else { 2001 };
                if dist + newdist < distances[((newx * grid[0].len() + newy) << 2) | *i as usize] {
                    distances[((newx * grid[0].len() + newy) << 2) | *i as usize] = dist + newdist;
                    pq.push((Reverse(dist + newdist), *i, newx, newy));
                    parent[((newx * grid[0].len() + newy) << 2) | *i as usize] = vec![(x, y, dir)];
                }
                else if dist + newdist == distances[((newx * grid[0].len() + newy) << 2) | *i as usize] {
                    parent[((newx * grid[0].len() + newy) << 2) | *i as usize].push((x, y, dir));
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((end.0, end.1, 0 as u8));
    queue.push_back((end.0, end.1, 1 as u8));
    queue.push_back((end.0, end.1, 2 as u8));
    queue.push_back((end.0, end.1, 3 as u8));
    let mut unique_pairs = HashSet::new();

    while !queue.is_empty() {
        let (x, y, dir) = queue.pop_front().unwrap();
        unique_pairs.insert((x, y));
        for par in &parent[((x * grid[0].len() + y) << 2) | dir as usize] {
            queue.push_back(*par);
        }
    }

    return unique_pairs.len().to_string();
}
