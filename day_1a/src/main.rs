use std::fs;
use std::env;

fn get_input() -> Vec<String> {
    env::var("TEST_INPUT").ok().unwrap_or_else(|| {
        fs::read_to_string("input.txt").unwrap_or_else(|_| {
            eprintln!("Error: input.txt not found.");
            std::process::exit(1);
        })
    })
    .lines()
    .map(|line| line.trim().to_string())
    .collect()
}

fn process_input<T>(input: Vec<String>, _type: &str) -> Vec<Vec<T>> 
where
    T: std::str::FromStr, 
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|word| word.parse::<T>().ok())
                .collect::<Vec<T>>()
        })
        .collect()
}

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

fn main() {
    let input = get_input();
    let mut processed_input = process_input::<u32>(input, "u32");
    
    sort_column(&mut processed_input);
    let answer = calculate_diff(&processed_input);

    println!("{}", answer);
}
