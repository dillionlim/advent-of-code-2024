use std::fs;
use std::env;
use toml::de::from_str;
use serde::Deserialize;
use walkdir::WalkDir;
use regex::Regex;

#[path="../src/common.rs"]
mod common;

#[path="../src/import.rs"]
mod import;

#[derive(Deserialize)]
struct TestCase {
    input: String,
    output: String,
}

#[derive(Deserialize)]
struct TestCaseVec {
    test_cases: Option<Vec<TestCase>>,
}

fn extract_part(path: &str) -> Option<String> {
    let re = Regex::new(r".*day_(\d+[ab]).*").unwrap();
    if let Some(captures) = re.captures(path) {
        Some(captures[1].to_string())
    } else {
        None
    }
}

#[test]
fn run_all_toml_test_cases() -> Result<(), String> {
    let mut failed_directories = Vec::new(); // Store directories with failed test cases
    let mut all_failed = false; // Track overall failure

    let commands: std::collections::HashMap<String, fn() -> String> = import::get_day_solvers();
    
    for entry in WalkDir::new("src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "test_cases.toml"){
        let path = entry.path();
        let dir = path.parent().unwrap_or_else(|| entry.path());
        let mut dir_failed = false;

        let file_content = fs::read_to_string(path)
            .expect("Unable to read test_cases.toml");

        let test_cases: TestCaseVec = from_str(&file_content)
            .expect("Invalid TOML format");

        let part = extract_part(path.to_str().expect("Unable to convert to string"))
            .expect("Unable to extract part from path");

        if let Some(test_cases) = test_cases.test_cases {
            for (i, case) in test_cases.iter().enumerate() {

                env::set_var("TEST_INPUT", &case.input);

                if let Some(solver) = &commands.get(&part) {
                    let result = solver();  
                    
                    let expected_output = case.output.trim();
            
                    if result != expected_output {
                        eprintln!(
                            "Test case {} failed in {}\n input: \"{}\"\n expected: \"{}\"\n output: \"{}\"\n",
                            i + 1, path.display(), case.input.trim(), expected_output, result
                        );
                        dir_failed = true;
                        all_failed = true;
                    }
                } else {
                    eprintln!("No solver found for test case {} in {}", i + 1, path.display());
                    dir_failed = true;
                    all_failed = true;
                }        
            }
        }

        if dir_failed {
            failed_directories.push(dir.display().to_string());
        }
    }

    if !failed_directories.is_empty() {
        eprintln!("\nThe following directories had test failures:");
        for dir in &failed_directories {
            eprintln!("  - {}", dir);
        }
    }

    if all_failed {
        return Err("One or more tests failed".to_string());
    }

    Ok(())
}
