// Auto-generated import.rs file
#[path = "day_1/day_1a/1a.rs"]
mod day_1a;
#[path = "day_1/day_1b/1b.rs"]
mod day_1b;

pub fn get_day_solvers() -> std::collections::HashMap<String, fn() -> String> {
    let mut map = std::collections::HashMap::new();
    map.insert("1a".to_string(), day_1a::solve as fn() -> String);
    map.insert("1b".to_string(), day_1b::solve as fn() -> String);
    map
}
