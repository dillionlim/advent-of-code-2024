// Implementation for Day 15, Part B
use crate::common::{get_input, process_input};
use std::path::Path;
use std::collections::VecDeque;

fn is_possible_to_push(grid: &Vec<Vec<char>>, x: isize, y: isize, dx: isize) -> bool {
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();

    queue.push_back((x, y));
    while !queue.is_empty() {
        let (curr_x, curr_y) = queue.pop_front().unwrap();

        let check_x = curr_x + dx;
        let check_y_left = curr_y;

        // case of []
        //         []

        if grid[check_x as usize][check_y_left as usize] == '[' {
            queue.push_back((check_x, check_y_left));
        }

        // case of [][]
        //          []

        else {
            if grid[check_x as usize][check_y_left as usize - 1] == '[' {
                queue.push_back((check_x, check_y_left - 1)); // the first [
            }
            if grid[check_x as usize][check_y_left as usize + 1] == '[' {
                queue.push_back((check_x, check_y_left + 1)); // the second [
            }

            if grid[check_x as usize][check_y_left as usize] == '#' ||
            grid[check_x as usize][check_y_left as usize + 1] == '#' {
                return false;
            }
        }
    }
    true
}

fn push(grid: &mut Vec<Vec<char>>, x: isize, y: isize, dx: isize) {
    let next_x = x + dx;
    
    // case of []
    //         []
    if grid[next_x as usize][y as usize] == '[' {
        push(grid, next_x, y, dx); // update all boxes above it first
        // then update this box, below
    }

    // case of [][]    []
    //          []    [][]
    //          ^     ^ 
    else {
        if grid[next_x as usize][y as usize - 1] == '[' {
            push(grid, next_x, y - 1, dx); // the first [
        }
        if grid[next_x as usize][y as usize + 1] == '[' {
            push(grid, next_x, y + 1, dx); // the second [
        }
        // update all boxes above it first
        // then update this box, below
    }

    // nothing above this box, just push the current box

    grid[next_x as usize][y as usize] = '[';
    grid[next_x as usize][y as usize + 1] = ']';
    grid[x as usize][y as usize] = '.';
    grid[x as usize][y as usize + 1] = '.';
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut split_iter = input.split(|line| line.is_empty());
    let input_grid = split_iter.next().unwrap().to_vec();
    let movements = split_iter.next().unwrap().to_vec();
    let original_grid: Vec<Vec<char>> = process_input(input_grid, "char");
    let mut grid: Vec<Vec<char>> = 
    original_grid
    .into_iter()
    .map(|row| {
        row.into_iter().flat_map(|cell| match cell {
            '#' => vec!['#', '#'],
            '.' => vec!['.', '.'],
            'O' => vec!['[', ']'],
            '@' => vec!['@', '.'],
            _ => vec![cell],
        })
        .collect()
    })
    .collect();

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
        else if grid[new_x as usize][new_y as usize] == '[' || 
        grid[new_x as usize][new_y as usize] == ']' {
            if dx == 0 { // moving left, right
                let mut temp_y = new_y;
                loop {
                    temp_y += dy;
                    if grid[x as usize][temp_y as usize] == '#' {
                        break;
                    } 
                    else if grid[x as usize][temp_y as usize] == '.' {
                        if dy > 0 {
                            for i in new_y+1..=temp_y {
                                if (i-new_y) % 2 == 0 {
                                    grid[x as usize][i as usize] = ']';
                                }
                                else {
                                    grid[x as usize][i as usize] = '[';
                                }
                            }
                        }
                        else {
                            for i in temp_y..new_y {
                                if (i-temp_y) % 2 == 0 {
                                    grid[x as usize][i as usize] = '[';
                                }
                                else {
                                    grid[x as usize][i as usize] = ']';
                                }
                            }
                        }
                        grid[x as usize][new_y as usize] = '.';
                        y = new_y;
                        break;
                    }
                }
            }
            else { // moving up and down
                let y_offset = if grid[new_x as usize][y as usize] == '[' { 0 } else { -1 };
                if is_possible_to_push(&grid, new_x, y + y_offset, dx) { 
                    // always simulate push from the [ side to make life easier
                    push(&mut grid, new_x, y + y_offset, dx);
                    x += dx;
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
            if v == '[' { Some(i * 100 + j) } else { None }
        })
    })
    .sum::<usize>();
    
    return ans.to_string();
}
