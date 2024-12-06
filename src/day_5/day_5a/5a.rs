// Implementation for Day 5, Part A
use crate::common::get_input;
use std::path::Path;
use std::collections::HashSet;

pub fn parse_input(input: &Vec<String>) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let mut part_1: HashSet<(u32, u32)> = HashSet::new();
    let mut part_2: Vec<Vec<u32>> = Vec::new();
    let mut is_second_part = false;

    for line in input {
        if line.trim().is_empty() {
            is_second_part = true;
            continue;
        }
        if is_second_part { // part 2
            let values: Vec<u32> = line.split(',')
                .filter_map(|x| x.trim().parse::<u32>().ok())
                .collect();
            part_2.push(values);
        } else { // part 1
            if let Some((a, b)) = line.split_once('|') {
                if let (Ok(a), Ok(b)) = (a.trim().parse::<u32>(), b.trim().parse::<u32>()) {
                    part_1.insert((a, b));
                }
            }
        }
    }

    (part_1, part_2)
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let (rules, updates) = parse_input(&input);

    let mut tot_middle = 0;

    for update in updates {
        if update.windows(2).all(|x| rules.contains(&(x[0], x[1]))) {
            tot_middle += update[(update.len()) / 2];
        }
    }
    
    return tot_middle.to_string();
}
