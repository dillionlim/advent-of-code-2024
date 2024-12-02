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
