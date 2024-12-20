// Implementation for Day 20, Part B
use crate::common::{get_input, process_input};
use std::path::Path;

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    // let processed_input: Vec<Vec<i32>> = process_input(input, "i32");
    
    return input.join("\n");
}
