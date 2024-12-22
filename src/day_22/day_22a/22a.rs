// Implementation for Day 22, Part A
use crate::common::get_input;
use std::path::Path;

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let secret_numbers = input.iter().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    
    let mut tot_ans = 0;

    for number in secret_numbers {
        let mut ans = number;
        for _ in 0..2000 {
            ans ^= ans << 6;
            ans &= 0xffffff;
            ans ^= ans >> 5;
            ans &= 0xffffff;
            ans ^= ans << 11;
            ans &= 0xffffff;
        }
        tot_ans += ans;
    }

    return tot_ans.to_string();
}
