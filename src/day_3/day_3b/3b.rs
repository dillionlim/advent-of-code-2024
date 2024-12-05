// Implementation for Day 3, Part B
use crate::common::get_input;
use std::path::Path;
use regex::Regex;

fn get_muls(line: &String, mut enabled: bool) -> (u32, bool) {
    let re = Regex::new(r"(?:(do\(\))|(don't\(\))|(?:mul\((\d+),(\d+)\)))").unwrap();
    let mut total: u32 = 0;
    for commands in re.captures_iter(line) {
        match (commands.get(1), commands.get(2),commands.get(3),commands.get(4)) { // do, don't, mul
            (Some(_), None, None, None) => enabled = true, // do
            (None, Some(_), None, None) => enabled = false, // don't
            (None, None, Some(first_no), Some(second_no)) => { // mul(first_no, second_no)
                let first_number = first_no.as_str().parse::<u32>().expect("Failed to extract first digit");
                let second_number = second_no.as_str().parse::<u32>().expect("Failed to extract second digit");
                
                total += first_number * second_number * if enabled == true { 1 } else { 0 };
            },
            _ => { // should not encounter this case
                println!("Should not encounter this case");
                ()
            } 
        }
    }
    (total, enabled)
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    
    let mut total_sum = 0;
    let mut enabled: bool = true;
    
    for line in input {
        let (sum_extracted_numbers, enabled_res): (u32, bool) = get_muls(&line, enabled);
        enabled = enabled_res;
        total_sum += sum_extracted_numbers;
    }
    
    return total_sum.to_string();
}
