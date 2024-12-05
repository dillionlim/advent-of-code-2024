use std::fs;
use std::env;
use std::path::{Path, PathBuf};

/*                       Input / Processing Inputs                       */

pub fn get_input(base_path: PathBuf) -> Vec<String> {
    let input_file_path = Path::new(&base_path).join("input.txt");
    env::var("TEST_INPUT").ok().unwrap_or_else(|| {
        fs::read_to_string(input_file_path).unwrap_or_else(|_| {
            eprintln!("Error: input.txt not found.");
            std::process::exit(1);
        })
    })
    .lines()
    .map(|line| line.trim().to_string())
    .collect()
}

pub fn process_input<T>(input: Vec<String>, _type: &str) -> Vec<Vec<T>> 
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