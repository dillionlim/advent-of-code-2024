// Auto-generated import.rs file
#[path = "day_1/day_1a/1a.rs"]
mod day_1a;
#[path = "day_1/day_1b/1b.rs"]
mod day_1b;
#[path = "day_2/day_2a/2a.rs"]
mod day_2a;
#[path = "day_2/day_2b/2b.rs"]
mod day_2b;
#[path = "day_3/day_3a/3a.rs"]
mod day_3a;
#[path = "day_3/day_3b/3b.rs"]
mod day_3b;

pub fn get_day_solvers() -> std::collections::HashMap<String, fn() -> String> {
    let mut map = std::collections::HashMap::new();
    map.insert("1a".to_string(), day_1a::solve as fn() -> String);
    map.insert("1b".to_string(), day_1b::solve as fn() -> String);
    map.insert("2a".to_string(), day_2a::solve as fn() -> String);
    map.insert("2b".to_string(), day_2b::solve as fn() -> String);
    map.insert("3a".to_string(), day_3a::solve as fn() -> String);
    map.insert("3b".to_string(), day_3b::solve as fn() -> String);
    map
}
