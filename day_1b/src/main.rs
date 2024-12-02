use std::fs;
use std::env;
use std::collections::HashMap;

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

fn main() {
    let input = get_input();
    let processed_input = process_input::<u32>(input, "u32");

    let counts = count_right(&processed_input);
    let answer = get_sum(&processed_input, &counts);

    println!("{}", answer);
}
