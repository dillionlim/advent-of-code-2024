// Implementation for Day 15, Part A
use crate::common::{get_input, process_input};
use std::path::Path;

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut split_iter = input.split(|line| line.is_empty());
    let input_grid = split_iter.next().unwrap().to_vec();
    let movements = split_iter.next().unwrap().to_vec();
    let mut grid: Vec<Vec<char>> = process_input(input_grid, "char");
    let movements = movements.join("");

    let (mut x, mut y) = grid
    .iter()
    .enumerate()
    .flat_map(|(i, row)| {
        row
        .iter()
        .enumerate()
        .filter_map(move |(j, &v)| {
            if v == '@' { Some((i as isize, j as isize)) } else { None }
        })
    })
    .next()
    .unwrap();

    grid[x as usize][y as usize] = '.';

    for movement in movements.chars() {
        let (dx, dy) = match movement {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!("Invalid movement: {}", movement),
        };
        let new_x = x + dx;
        let new_y = y + dy;
        if grid[new_x as usize][new_y as usize] == '.' {
            x = new_x;
            y = new_y;
        }
        else if grid[new_x as usize][new_y as usize] == 'O' {
            let mut temp_x = new_x;
            let mut temp_y = new_y;
            loop {
                temp_x += dx;
                temp_y += dy;
                if grid[temp_x as usize][temp_y as usize] == '#' {
                    break;
                } 
                else if grid[temp_x as usize][temp_y as usize] == '.' {
                    grid[temp_x as usize][temp_y as usize] = 'O';
                    grid[new_x as usize][new_y as usize] = '.';
                    x = new_x;
                    y = new_y;
                    break;
                }
            }
        }
    }

    let ans = grid
    .iter()
    .enumerate()
    .flat_map(|(i, row)| {
        row
        .iter()
        .enumerate()
        .filter_map(move |(j, &v)| {
            if v == 'O' { Some(i * 100 + j) } else { None }
        })
    })
    .sum::<usize>();
    
    return ans.to_string();
}
