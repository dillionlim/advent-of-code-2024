// Implementation for Day 17, Part B
use crate::common::get_input;
use std::path::Path;
use std::collections::HashSet;

fn process_input(input: &Vec<String>) -> Vec<u8> {
    let mut program = Vec::new();
    for line in input {
        if line.starts_with("Program:") {
            program = line
            .split_once(":")
            .unwrap().1
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect(); 
        }
    }

    program
}

fn get_output_val(a: usize) -> usize {
    let c = (a % 8) ^ 2;
    (c ^ ((a >> c) ^ 3)) % 8
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let program: Vec<u8> = process_input(&input);
    
    let mut prev: HashSet<usize> = HashSet::new();
    prev.insert(0);

    for target_value in program.iter().rev() {
        let mut curr: HashSet<usize> = HashSet::new();
        for prev_possible in prev {
            for i in 0usize..8 {
                let curr_test = (prev_possible << 3) + i as usize;
                if get_output_val(curr_test) == *target_value as usize {
                    curr.insert(curr_test);
                }
            }
        }
        prev = curr;
    }

    return prev.iter().min().unwrap_or(&0).to_string();
}
