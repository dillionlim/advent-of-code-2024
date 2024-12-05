// Implementation for Day 2, Part A
use crate::common::{get_input, process_input};
use std::path::Path;

fn process_row(row: &Vec<u32>) -> bool {
    if row.len() < 2 {
        return true; // trivially safe
    }
    let increasing = row[0] <= row[1];
    let mindiff = 1;
    let maxdiff = 3;
    row
    .windows(2)
    .all(|w| {
            let diff = w[1] as i32 - w[0] as i32;
            if (w[0] <= w[1]) != increasing {
                return false;
            }
            (diff.abs() <= maxdiff) && (diff.abs() >= mindiff)
        } 
    )
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input = process_input::<u32>(input, "u32");

    let answer: usize = processed_input.iter().map(|row| process_row(&row)).filter(|res| *res).count();
    
    return answer.to_string();
}
