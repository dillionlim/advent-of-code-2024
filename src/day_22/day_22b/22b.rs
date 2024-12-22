// Implementation for Day 22, Part B
use crate::common::get_input;
use std::path::Path;

fn next(number: u64) -> u64 {
    let mut n = number;
    n ^= n << 6;
    n &= 0xffffff;
    n ^= n >> 5;
    n &= 0xffffff;
    n ^= n << 11;
    n &= 0xffffff;
    n
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let secret_numbers = input.iter().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    
    let mut sequences: Vec<u64> = vec![0; 130321]; // 19^4 = 130321 possible sequences
    let mut vis = vec![std::u8::MAX; 130321];

    for (i, &number) in secret_numbers.iter().enumerate() {
        let mut last_no = number;
        let mut curr_no = next(last_no);
        let mut hash = 0i64;
        // 19 ^ 4 = 130321
        const MODULO_BASE_19: i64 = 130321;
        // 9 * 19^3 + 9 * 19^2 + 9 * 19^1 + 9 * 19^0 = 65160
        const OFFSET_BASE_19: i64 = 65160;
        for _ in 0..3 {
            hash = (hash * 19 + (curr_no % 10) as i64 - (last_no % 10) as i64 + OFFSET_BASE_19) % MODULO_BASE_19;
            last_no = curr_no;
            curr_no = next(curr_no);
        }
        for _ in 3..1999 {
            hash = (hash * 19 + (curr_no % 10) as i64 - (last_no % 10) as i64 + OFFSET_BASE_19) % MODULO_BASE_19;
            if vis[hash as usize] != i as u8 {
                vis[hash as usize] = i as u8;
                sequences[hash as usize] += curr_no % 10;
            }
            last_no = curr_no;
            curr_no = next(curr_no);
        }
    }

    return sequences.iter().max().unwrap().to_string();
}
