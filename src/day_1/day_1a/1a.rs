// Implementation for Day 1, Part A
use crate::common::{get_input, process_input};
use std::path::Path;

fn sort_column(input: &mut Vec<Vec<u32>>) {
    if input.is_empty() {
        return;
    }
    if input[0].len() != 2 {
        eprintln!("Error: Each row should contain exactly two numbers.");
        return;
    }

    for col in 0..2 {
        let mut col_input: Vec<u32> = input.iter().map(|row| row[col]).collect();
        col_input.sort();
        for (i, row) in input.iter_mut().enumerate() {
            row[col] = col_input[i];
        }
    }
}

fn calculate_diff(input: &Vec<Vec<u32>>) -> u32 {
    let answer: u32 = input.iter()
                                     .map(|row| (row[0]).abs_diff(row[1]))
                                     .sum::<u32>();
    return answer;
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut processed_input = process_input::<u32>(input, "u32");
    
    sort_column(&mut processed_input);
    let answer = calculate_diff(&processed_input);
    
    return answer.to_string();
}
