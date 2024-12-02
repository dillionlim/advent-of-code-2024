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

fn main() {
    let input = get_input();
    let processed_input = process_input::<u32>(input, "u32");

    let answer: usize = processed_input.iter().map(|row| process_row(&row)).filter(|res| *res).count();

    println!("{}", answer);
}
