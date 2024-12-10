use std::fs;
use std::process::Command;
use std::time::Instant;
use std::path::Path;

mod common;
mod import;

fn format_duration(duration: std::time::Duration) -> String {
    if duration.as_micros() < 1_000 {
        (duration.as_micros().to_string() + " μs").to_string()
    } else if duration.as_micros() < 1_000_000 {
        (duration.as_millis().to_string() + " ms").to_string()
    } else {
        (duration.as_secs().to_string() + " s").to_string()
    }
}

fn run_cargo_test(day: &str) -> bool {
    let output = Command::new("cargo")
        .args(&["test", "--quiet"])
        .current_dir(format!("src/day_{}", day))
        .output()
        .expect("Failed to execute tests");
    output.status.success()
}

fn run_cargo_run(day: &str, part: &str, commands: &std::collections::HashMap<String, fn() -> String>) -> (String, std::time::Duration) {
    let key = format!("{}{}", day, part);
    
    if let Some(solver) = &commands.get(&key) {
        let start_time = Instant::now();
        let result = solver();
        let duration = start_time.elapsed();
        
        (result, duration)
    } else {
        ("Unknown".to_string(), std::time::Duration::new(0, 0))
    }
}

fn extract_problem_name(day: &str) -> String {
    let explanation_path = format!("src/day_{}/explanation.md", day);
    if Path::new(&explanation_path).exists() {
        let content = fs::read_to_string(&explanation_path).unwrap_or_default();
        content.lines()
            .next()
            .unwrap_or("")
            .replace("# Day", "")
            .trim()
            .to_string()
    } else {
        format!("Day {}", day)
    }
}

fn print_table(days: Vec<String>) {
    let mut results = vec![];
    let commands = import::get_day_solvers();

    // Collecting the results for each day
    for day in days {
        let day_number = day.parse::<usize>().unwrap();
        let problem_name = extract_problem_name(&day);
        let (part1_result, part1_time) = run_cargo_run(&day, "a", &commands);
        let (part2_result, part2_time) = run_cargo_run(&day, "b", &commands);
        let tests_passed = run_cargo_test(&day);

        results.push((
            day_number,
            problem_name,
            part1_result,
            part1_time,
            tests_passed,
            part2_result,
            part2_time,
        ));
    }

    results.sort_by(|a, b| a.0.cmp(&b.0));

    // Calculate maximum lengths for columns
    let max_name_len = results.iter().map(|r| r.1.len()).max().unwrap_or(3).max(3);
    let max_part1_len = results.iter().map(|r| r.2.len() ).max().unwrap_or(6).max(6);
    let max_part1_time_len = results.iter().map(|r| format_duration(r.3).len()).max().unwrap_or(4).max(4);
    let max_part2_len = results.iter().map(|r| r.5.len() ).max().unwrap_or(6).max(6);
    let max_part2_time_len = results.iter().map(|r| format_duration(r.6).len()).max().unwrap_or(4).max(4);

    // Calculate the total width for the table header
    let name_width = max_name_len + 2;
    let part1_width = max_part1_len + 2;
    let part1_time_width = max_part1_time_len + 2;
    let part2_width = max_part2_len + 2;
    let part2_time_width = max_part2_time_len + 2;

    let max_total_len = name_width + part1_width + part1_time_width + part2_width + part2_time_width + 12;

    let title = "🦀 Advent of Code 2024 🦀";

    println!("╔{}╗", "═".repeat(max_total_len+2));
    println!("║{:^max_total_len$}║", title);

    // Print the top border
    println!(
        "╠{:═^name_width$}╦{:═^part1_width$}╦{:═^part1_time_width$}╦{:═^4}╦{:═^part2_width$}╦{:═^part2_time_width$}╦{:═^4}╣",
        "", "", "", "", "", "", ""
    );

    // Print the header row
    println!(
        "║{:^name_width$}║{:^part1_width$}║{:^part1_time_width$}║{:^4}║{:^part2_width$}║{:^part2_time_width$}║{:^4}║",
        "Day", "Part 1", "Time", "Test", "Part 2", "Time", "Test"
    );

    // Print the separator line
    println!(
        "╠{:═^name_width$}╬{:═^part1_width$}╬{:═^part1_time_width$}╬{:═^4}╬{:═^part2_width$}╬{:═^part2_time_width$}╬{:═^4}╣",
        "", "", "", "", "", "", ""
    );

    // Print the table rows for each day
    for result in results {
        let (_, name, part1, part1_time, test1, part2, part2_time) = result;
        println!(
            "║{:^name_width$}║{:^part1_width$}║{:^part1_time_width$}║{:^3}║{:^part2_width$}║{:^part2_time_width$}║{:^3}║",
            name,
            part1,
            format_duration(part1_time),
            if test1 { "✅" } else { "❌" },
            part2,
            format_duration(part2_time),
            if test1 { "✅" } else { "❌" }
        );
    }

    // Print the bottom border
    println!(
        "╚{:═^name_width$}╩{:═^part1_width$}╩{:═^part1_time_width$}╩{:═^4}╩{:═^part2_width$}╩{:═^part2_time_width$}╩{:═^4}╝",
        "", "", "", "", "", "", ""
    );
}

fn main() {
    let days: Vec<String> = fs::read_dir("src")
        .expect("Failed to read src directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().into_string().ok()?;
            if file_name.starts_with("day_") {
                Some(file_name.replace("day_", ""))
            } else {
                None
            }
        })
        .collect();

    print_table(days);
}
