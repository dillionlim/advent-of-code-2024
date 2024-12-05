// Implementation for Day 3, Part A
use crate::common::get_input;
use std::path::Path;
use regex::Regex;

fn get_muls(line: &String) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re
    .captures_iter(line)
    .map(|mul| {
        let first_number = mul[1].parse::<u32>().expect("Failed to extract first digit");
        let second_number = mul[2].parse::<u32>().expect("Failed to extract second digit");
        first_number * second_number
    })
    .sum()
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);

    let mut total_sum = 0;
    
    for line in input {
        let sum_extracted_numbers: u32 = get_muls(&line);
        total_sum += sum_extracted_numbers;
    }
    
    return total_sum.to_string();
}
