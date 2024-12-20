// Implementation for Day 20, Part A
use crate::common::{get_input, process_input};
use std::path::Path;

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<Vec<char>> = process_input(input, "char");
    
    
    return input.join("\n");
}
