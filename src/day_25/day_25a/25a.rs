// Implementation for Day 25, Part A
use crate::common::get_input;
use std::path::Path;

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut input_locks: Vec<u64> = Vec::new();
    let mut input_keys: Vec<u64> = Vec::new();
    input
    .split(|line| line.is_empty())
    .for_each(|grid| {
        let mut hashed_val: u64 = 0;
        for line in grid {
            for &chars in line.as_bytes() {
                hashed_val <<= 1;
                if chars == b'#' {
                    hashed_val |= 1;
                }
            }
        }
        if hashed_val & 1 == 0 {
            input_keys.push(hashed_val);
        }
        else {
            input_locks.push(hashed_val);
        }
    });

    let mut ans = 0;
    for key in input_keys {
        for &lock in &input_locks {
            let lower_35_bits_mask: u64 = (1 << 36) - 1;
            let overlap = (key & lock) & lower_35_bits_mask;
            if overlap == 0 {
                ans += 1;
            }
        }
    }
    
    return ans.to_string();
}
