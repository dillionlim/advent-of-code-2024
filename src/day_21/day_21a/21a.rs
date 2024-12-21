// Implementation for Day 21, Part A
use crate::common::get_input;
use std::path::Path;
use std::collections::{VecDeque, HashMap};

// represent A as 10
// represent as (number, direction)
// dir is 1, 2, 3, 4 = up, right, down, left

const ADJACENCY_LIST_KEYPAD: &[&[(usize, u8)]] = &[
    &[(2, 1), (10, 2)], 
    &[(2, 2), (4, 1)], 
    &[(0, 3), (1, 4), (3, 2), (5, 1)], 
    &[(2, 4), (6, 1), (10, 3)], 
    &[(1, 3), (5, 2), (7, 1)], 
    &[(2, 3), (4, 4), (6, 2), (8, 1)], 
    &[(3, 3), (5, 4), (9, 1)], 
    &[(4, 3), (8, 2)], 
    &[(5, 3), (7, 4), (9, 2)], 
    &[(6, 3), (8, 4)],
    &[(0, 4), (3, 1)]
];

// represent ^ as 0, > as 1, v as 2, < as 3, A as 4
// represent as (number, direction)
// dir is 1, 2, 3, 4 = up, right, down, left
const ADJACENCY_LIST_DIRPAD: &[&[(usize, u8)]] = &[
    &[(2, 3), (4, 2)], 
    &[(2, 4), (4, 1)], 
    &[(0, 1), (1, 2), (3, 4)], 
    &[(2, 2)], 
    &[(0, 4), (1, 3)]
];

fn find_shortest_paths_numpad(character: &u8, starting_char: &u8) -> Vec<Vec<u32>> {
    let mut queue = VecDeque::new();
    let mut dist = vec![-1; 11];
    let mut parents: Vec<Vec<(usize, u8)>> = vec![Vec::new(); 11];

    queue.push_back(*starting_char as usize);
    dist[*starting_char as usize] = 0;

    while !queue.is_empty() {
        let sz = queue.len();
        for _ in 0..sz {
            let curr = queue.pop_front().unwrap();
            if curr == *character as usize {
                let mut stored_paths = Vec::new();
                let mut backtracking = VecDeque::new();
                backtracking.push_back((*character, 0u32));
                while let Some((curr, prev_dirs)) = backtracking.pop_front() {
                    if curr == *starting_char {
                        let mut path = vec![];
                        let mut curr_path = prev_dirs;
                        while curr_path > 0 {
                            path.push((curr_path % 10) - 1);
                            curr_path /= 10;
                        }
                        path.push(4);
                        stored_paths.push(path);
                    }
                    for &(parent, dir) in parents[curr as usize].iter() {
                        let curr_dir = prev_dirs * 10 + dir as u32;
                        backtracking.push_back((parent as u8, curr_dir));
                    }
                }
                
                return stored_paths;
            }
            for &(neighbour, dir) in ADJACENCY_LIST_KEYPAD[curr] {
                if dist[neighbour] == -1 {
                    dist[neighbour] = dist[curr] + 1;
                    queue.push_back(neighbour);
                    parents[neighbour].push((curr, dir));
                }
                else if dist[neighbour] == dist[curr] + 1 {
                    parents[neighbour].push((curr, dir));
                }
            }
        }
    }

    unreachable!()
}

fn hash_pos_robots(curr: u8, next: u8, robots: u8) -> u32 {
    (curr as u32) | ((next as u32) << 8) | ((robots as u32) << 16)
}

fn find_shortest_len_rec(curr: u8, next: u8, robots: u8, dp: &mut HashMap<u32, u32>) -> u32 {
    if dp.contains_key(&hash_pos_robots(curr, next, robots)) {
        return *dp.get(&hash_pos_robots(curr, next, robots)).unwrap();
    }

    let mut queue = VecDeque::new();
    let mut dist = vec![-1; 5];
    let mut parents: Vec<Vec<(usize, u8)>> = vec![Vec::new(); 5];

    queue.push_back(curr);
    dist[curr as usize] = 0;

    while !queue.is_empty() {
        let sz = queue.len();
        for _ in 0..sz {
            let curr_pos = queue.pop_front().unwrap();
            if curr_pos == next {
                let mut backtracking = VecDeque::new();
                backtracking.push_back((next, 0u32));
                let mut answer = std::u32::MAX;
                while let Some((curr_node, prev_dirs)) = backtracking.pop_front() {
                    if curr_node == curr {
                        let mut path = vec![];
                        let mut curr_path = prev_dirs;
                        while curr_path > 0 {
                            path.push((curr_path % 10) - 1);
                            curr_path /= 10;
                        }
                        path.push(4);

                        answer = answer.min(find_shortest_len_dirpad(&path, robots - 1, dp));
                    }
                    for &(parent, dir) in parents[curr_node as usize].iter() {
                        let curr_dir = prev_dirs * 10 + dir as u32;
                        backtracking.push_back((parent as u8, curr_dir));
                    }
                }

                dp.insert(hash_pos_robots(curr, next, robots), answer);
                return answer;
            }
            for &(neighbour, dir) in ADJACENCY_LIST_DIRPAD[curr_pos as usize] {
                if dist[neighbour] == -1 {
                    dist[neighbour] = dist[curr_pos as usize] + 1;
                    queue.push_back(neighbour as u8);
                    parents[neighbour].push((curr_pos as usize, dir));
                }
                else if dist[neighbour] == dist[curr_pos as usize] + 1 {
                    parents[neighbour].push((curr_pos as usize, dir));
                }
            }
        }
    }

    unreachable!()
}

fn find_shortest_len_dirpad(directions: &Vec<u32>, robots: u8, dp: &mut HashMap<u32, u32>) -> u32 {
    if robots == 0 {
        return directions.len() as u32;
    }
    let mut curr = 4;
    let mut tot_len = 0;
    for dir in directions {
        tot_len += find_shortest_len_rec(curr, *dir as u8, robots, dp);
        curr = *dir as u8;
    }
    tot_len
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let codes: Vec<&[u8]> = input.iter().map(|s| s.as_bytes()).collect();

    let mut ans = 0;

    for code in codes {
        let numeric_part = std::str::from_utf8(&code[0..=2]).unwrap().parse::<u32>().unwrap();
        let mut tot_len_of_path = 0;

        // obtain all shortest paths to get the first code
        let mut prev = 10;
        let mut dp: HashMap<u32, u32> = HashMap::new();
        for character in code.iter() {
            let curr = if *character == b'A' { 10 } else { character - b'0' };
            let shortest_paths = find_shortest_paths_numpad(&curr, &prev);
            tot_len_of_path += shortest_paths.iter().map(|path| find_shortest_len_dirpad(path, 2, &mut dp)).min().unwrap();
            prev = curr;
        }

        ans += tot_len_of_path * numeric_part;
    }
    
    return ans.to_string();
}
