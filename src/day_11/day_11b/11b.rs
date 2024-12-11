// Implementation for Day 11, Part B
use crate::common::{get_input, process_input};
use std::path::Path;
use hashbrown::HashMap;

fn count_digits (n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn split_number (n: u64, no_of_digits: u32) -> (u64, u64) {
    let d = 10_u64.pow(no_of_digits/2);
    (n / d, n % d)
}

fn process_iteration (counts_old: &HashMap<u64, usize>, counts_new: &mut HashMap<u64, usize>) {
    counts_new.reserve(counts_old.len());
    for (&stone, &count) in counts_old {
        if stone == 0 {
            *counts_new.entry(1).or_default() += count;
            continue;
        }
        let no_of_digits = count_digits(stone);
        if no_of_digits % 2 == 0 {
            let (left, right) = split_number(stone, no_of_digits);
            *counts_new.entry(left).or_default() += count;
            *counts_new.entry(right).or_default() += count;
        }
        else {
            *counts_new.entry(stone * 2024).or_default() += count;
        }
    }
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: &Vec<u64> = &process_input(input, "u64")[0];

    let mut counts_odd: HashMap<u64, usize> = HashMap::new();
    let mut counts_even: HashMap<u64, usize> = HashMap::new();

    processed_input
    .iter()
    .for_each(|&stone| *counts_even.entry(stone).or_default() += 1);

    let no_of_iterations = 75;

    for i in 0..no_of_iterations {
        if i % 2 == 0 {
            process_iteration(&mut counts_even, &mut counts_odd);
            counts_even.clear();
        }
        else {
            process_iteration(&mut counts_odd, &mut counts_even);
            counts_odd.clear();
        }
    }
    
    if no_of_iterations % 2 == 0 {
        counts_even.values().sum::<usize>().to_string()
    }
    else {
        counts_odd.values().sum::<usize>().to_string()
    }
}
