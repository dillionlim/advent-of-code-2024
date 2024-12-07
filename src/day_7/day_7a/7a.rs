// Implementation for Day 7, Part A
use crate::common::get_input;
use std::path::Path;
use std::collections::VecDeque;

fn process_input(input: Vec<String>) -> Vec<(u64, Vec<u64>)> {
    input
        .into_iter()
        .map(|line| {
                let mut numbers = line.split_whitespace();
                if let Some(first_number) = numbers.next() {
                    let trimmed_first_no = 
                    first_number
                    .trim_end_matches(':')
                    .parse::<u64>()
                    .expect("Failed to parse first number");
                    
                    return (trimmed_first_no, 
                            numbers
                            .filter_map(|word| word.parse::<u64>().ok())
                            .collect());
                }
                println!("Failed to parse line");
                (0, Vec::new())
            }
        )
        .collect()
}

fn is_line_possible(target: u64, numbers: Vec<u64>) -> bool {
    let mut queue: VecDeque<u64> = VecDeque::from([numbers[0]]);

    for number in numbers.iter().skip(1) {
        let no_of_elements = queue.len();
        for _ in 0..no_of_elements {
            let prev = queue.pop_front().expect("Failed to pop front from queue");
            if prev + number <= target {
                queue.push_back(prev + number);
            }
            if prev * number <= target {
                queue.push_back(prev * number);
            }
        }
    }

    queue.contains(&target)
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<(u64, Vec<u64>)> = process_input(input);

    let mut total_possible = 0;
    
    for (target, numbers) in processed_input {
        if is_line_possible(target, numbers) {
            total_possible += target;
        }
    }

    return total_possible.to_string();
}
