// Implementation for Day 1, Part B
use crate::common::{get_input, process_input};
use std::path::Path;
use std::collections::HashMap;

fn count_right(input: &Vec<Vec<u32>>) -> HashMap<u32, u32> {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    input
    .iter()
    .for_each(|row| *counts.entry(row[1]).or_insert(0) += 1);
    return counts;
}

fn get_sum(input: &Vec<Vec<u32>>, counts: &HashMap<u32, u32>) -> u32 {
    let answer = input.iter()
                      .map(|row| row[0]*counts.get(&row[0]).unwrap_or(&0))
                      .sum();
    return answer;
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input = process_input::<u32>(input, "u32");

    let counts = count_right(&processed_input);
    let answer = get_sum(&processed_input, &counts);
    
    return answer.to_string();
}
