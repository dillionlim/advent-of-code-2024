#!/bin/bash

usage() {
    echo "Usage: $0 -n <project_number>"
    exit 1
}

while getopts "n:" opt; do
    case "$opt" in
        n) project_number=$OPTARG ;;
        *) usage ;;
    esac
done

if [ -z "$project_number" ]; then
    usage
fi

project_name="day_$project_number"
echo "Creating project: $project_name"
cargo new $project_name || exit 1
cd $project_name || exit 1

echo " Creating explanation.md..."
cat > explanation.md <<EOL
# Day $project_number: 

## Abridged Problem Statement

## Solution

### Input Bounds

### Actual Solution

### Code Complexity

**Time Complexity:** $O(N)$

**Additional Space Complexity:** $O(N)$

**Final answer:** .
EOL

mkdir -p tests
echo "Creating test_cases.toml..."
cat > tests/test_cases.toml <<EOL
[[test_cases]]
input = "5"
output = "Input was: 5"

[[test_cases]]
input = """
multi
line
test
"""
output = """Input was: multi
line
test"""
EOL

echo "Creating testing suite..."
cat > tests/test_toml.rs <<EOL
use std::fs;
use std::process::Command;
use toml::de::from_str;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TestCase {
    input: String,
    output: String,
}

#[derive(Deserialize, Debug)]
struct TestCaseVec {
    test_cases: Vec<TestCase>,
}

#[test]
fn toml_test_cases() -> Result<(), String> {
    let file_content = fs::read_to_string("tests/test_cases.toml")
        .expect("Unable to read test_cases.toml");

    let test_cases: TestCaseVec = from_str(&file_content)
        .expect("Invalid TOML format");

    let mut failed = false;

    for (i, case) in test_cases.test_cases.iter().enumerate() {
        let output = Command::new("cargo")
            .args(&["run", "--quiet"])
            .env("TEST_INPUT", &case.input)
            .output()
            .expect("Failed to execute process");

        let stdout = String::from_utf8_lossy(&output.stdout);

        let expected_output = format!("{}\n", case.output.trim());

        if stdout != expected_output {
            eprintln!(
                "Test case {} failed\n input: \"{}\"\n expected: \"{}\"\n output: \"{}\"\n",
                i + 1, case.input.trim(), case.output.trim(), stdout.trim()
            );
            failed = true;
        }
    }

    if failed {
        return Err("Test(s) failed".to_string());
    }

    Ok(())
}
EOL

echo "Setting up main.rs for input/output handling..."
touch input.txt
cat > src/main.rs <<EOL
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

fn main() {
    let input = get_input();

    println!("Input was: {}", input.join("\n"));
}
EOL

echo "Adding serde and serde_toml dependencies..."
cat >> Cargo.toml <<EOL
serde = { version = "1.0", features = ["derive"] }
toml = "=0.8.19"
EOL

echo "Project setup complete. Navigate to $project_name and:"
echo "1. Write your code in src/main.rs."
echo "2. Add or modify test cases in tests/test_cases.toml."
echo "3. Run tests with: cargo test"
echo "4. Run the program with the sample input in input.txt with: cargo run"

exit 0
