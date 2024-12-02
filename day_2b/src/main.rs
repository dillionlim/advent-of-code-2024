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

fn get_direction(row: &Vec<u32>) -> bool {
    let mut increasingcnt = 0; 
    // if increasingcnt is >=2, then the whole sequence must be increasing otherwise decreasing
    for i in 0..3 {
        if row[i] <= row[i+1] {
            increasingcnt += 1;
        }
    }

    increasingcnt >= 2
}

fn process_row(row: &Vec<u32>) -> bool {
    if row.len() < 3 {
        return true; // trivially safe
    }
    if row.len() == 3 { // check if any 2 numbers are safe
        let range = 1..=3;
        return (0..3).any(|i| {
            range.contains(&((row[i] as i32 - row[(i + 1) % 3] as i32).abs()))
        });        
    }
    let increasing = get_direction(row);
    let mut dampener_active = true;
    let range = 1..=3;

    let mut i = 1;
    while i < row.len() {
        let diff = row[i] as i32 - row[i - 1] as i32;

        // This is an invalid pair
        if (row[i - 1] <= row[i]) != increasing || !range.contains(&diff.abs()) {
            if !dampener_active {
                return false; // Dampener already used, invalid sequence
            }

            dampener_active = false; // Use the dampener

            if i == row.len() - 1 { // End of row, just skip last element
                break;
            }

            // Check if removing the second element makes it valid
            if (row[i - 1] <= row[i + 1]) == increasing && range.contains(&(row[i + 1] as i32 - row[i - 1] as i32).abs()) {
                i += 2;
                continue;
            }

            // Check if removing the first element makes it valid
            if i > 1 { // If first element, just skip
                if (row[i - 2] <= row[i]) != increasing || !range.contains(&(row[i] as i32 - row[i - 2] as i32).abs()) {
                    return false; // Cannot salvage this pair
                }
            }
        }

        i += 1;
    }

    true
}

fn main() {
    let input = get_input();
    let processed_input = process_input::<u32>(input, "u32");

    let answer: usize = processed_input.iter().map(|row| process_row(&row)).filter(|res| *res).count();

    println!("{}", answer);
}
