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
#[path = "day_4/day_4a/4a.rs"]
mod day_4a;
#[path = "day_4/day_4b/4b.rs"]
mod day_4b;
#[path = "day_5/day_5a/5a.rs"]
mod day_5a;
#[path = "day_5/day_5b/5b.rs"]
mod day_5b;
#[path = "day_6/day_6a/6a.rs"]
mod day_6a;
#[path = "day_6/day_6b/6b.rs"]
mod day_6b;
#[path = "day_7/day_7a/7a.rs"]
mod day_7a;
#[path = "day_7/day_7b/7b.rs"]
mod day_7b;

pub fn get_day_solvers() -> std::collections::HashMap<String, fn() -> String> {
    let mut map = std::collections::HashMap::new();
    map.insert("1a".to_string(), day_1a::solve as fn() -> String);
    map.insert("1b".to_string(), day_1b::solve as fn() -> String);
    map.insert("2a".to_string(), day_2a::solve as fn() -> String);
    map.insert("2b".to_string(), day_2b::solve as fn() -> String);
    map.insert("3a".to_string(), day_3a::solve as fn() -> String);
    map.insert("3b".to_string(), day_3b::solve as fn() -> String);
    map.insert("4a".to_string(), day_4a::solve as fn() -> String);
    map.insert("4b".to_string(), day_4b::solve as fn() -> String);
    map.insert("5a".to_string(), day_5a::solve as fn() -> String);
    map.insert("5b".to_string(), day_5b::solve as fn() -> String);
    map.insert("6a".to_string(), day_6a::solve as fn() -> String);
    map.insert("6b".to_string(), day_6b::solve as fn() -> String);
    map.insert("7a".to_string(), day_7a::solve as fn() -> String);
    map.insert("7b".to_string(), day_7b::solve as fn() -> String);
    map
}
