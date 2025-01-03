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
#[path = "day_8/day_8a/8a.rs"]
mod day_8a;
#[path = "day_8/day_8b/8b.rs"]
mod day_8b;
#[path = "day_9/day_9a/9a.rs"]
mod day_9a;
#[path = "day_9/day_9b/9b.rs"]
mod day_9b;
#[path = "day_10/day_10a/10a.rs"]
mod day_10a;
#[path = "day_10/day_10b/10b.rs"]
mod day_10b;
#[path = "day_11/day_11a/11a.rs"]
mod day_11a;
#[path = "day_11/day_11b/11b.rs"]
mod day_11b;
#[path = "day_12/day_12a/12a.rs"]
mod day_12a;
#[path = "day_12/day_12b/12b.rs"]
mod day_12b;
#[path = "day_13/day_13a/13a.rs"]
mod day_13a;
#[path = "day_13/day_13b/13b.rs"]
mod day_13b;
#[path = "day_14/day_14a/14a.rs"]
mod day_14a;
#[path = "day_14/day_14b/14b.rs"]
mod day_14b;
#[path = "day_15/day_15a/15a.rs"]
mod day_15a;
#[path = "day_15/day_15b/15b.rs"]
mod day_15b;
#[path = "day_16/day_16a/16a.rs"]
mod day_16a;
#[path = "day_16/day_16b/16b.rs"]
mod day_16b;
#[path = "day_17/day_17a/17a.rs"]
mod day_17a;
#[path = "day_17/day_17b/17b.rs"]
mod day_17b;
#[path = "day_18/day_18a/18a.rs"]
mod day_18a;
#[path = "day_18/day_18b/18b.rs"]
mod day_18b;
#[path = "day_19/day_19a/19a.rs"]
mod day_19a;
#[path = "day_19/day_19b/19b.rs"]
mod day_19b;
#[path = "day_20/day_20a/20a.rs"]
mod day_20a;
#[path = "day_20/day_20b/20b.rs"]
mod day_20b;
#[path = "day_21/day_21a/21a.rs"]
mod day_21a;
#[path = "day_21/day_21b/21b.rs"]
mod day_21b;
#[path = "day_22/day_22a/22a.rs"]
mod day_22a;
#[path = "day_22/day_22b/22b.rs"]
mod day_22b;
#[path = "day_23/day_23a/23a.rs"]
mod day_23a;
#[path = "day_23/day_23b/23b.rs"]
mod day_23b;
#[path = "day_24/day_24a/24a.rs"]
mod day_24a;
#[path = "day_24/day_24b/24b.rs"]
mod day_24b;
#[path = "day_25/day_25a/25a.rs"]
mod day_25a;
#[path = "day_25/day_25b/25b.rs"]
mod day_25b;

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
    map.insert("8a".to_string(), day_8a::solve as fn() -> String);
    map.insert("8b".to_string(), day_8b::solve as fn() -> String);
    map.insert("9a".to_string(), day_9a::solve as fn() -> String);
    map.insert("9b".to_string(), day_9b::solve as fn() -> String);
    map.insert("10a".to_string(), day_10a::solve as fn() -> String);
    map.insert("10b".to_string(), day_10b::solve as fn() -> String);
    map.insert("11a".to_string(), day_11a::solve as fn() -> String);
    map.insert("11b".to_string(), day_11b::solve as fn() -> String);
    map.insert("12a".to_string(), day_12a::solve as fn() -> String);
    map.insert("12b".to_string(), day_12b::solve as fn() -> String);
    map.insert("13a".to_string(), day_13a::solve as fn() -> String);
    map.insert("13b".to_string(), day_13b::solve as fn() -> String);
    map.insert("14a".to_string(), day_14a::solve as fn() -> String);
    map.insert("14b".to_string(), day_14b::solve as fn() -> String);
    map.insert("15a".to_string(), day_15a::solve as fn() -> String);
    map.insert("15b".to_string(), day_15b::solve as fn() -> String);
    map.insert("16a".to_string(), day_16a::solve as fn() -> String);
    map.insert("16b".to_string(), day_16b::solve as fn() -> String);
    map.insert("17a".to_string(), day_17a::solve as fn() -> String);
    map.insert("17b".to_string(), day_17b::solve as fn() -> String);
    map.insert("18a".to_string(), day_18a::solve as fn() -> String);
    map.insert("18b".to_string(), day_18b::solve as fn() -> String);
    map.insert("19a".to_string(), day_19a::solve as fn() -> String);
    map.insert("19b".to_string(), day_19b::solve as fn() -> String);
    map.insert("20a".to_string(), day_20a::solve as fn() -> String);
    map.insert("20b".to_string(), day_20b::solve as fn() -> String);
    map.insert("21a".to_string(), day_21a::solve as fn() -> String);
    map.insert("21b".to_string(), day_21b::solve as fn() -> String);
    map.insert("22a".to_string(), day_22a::solve as fn() -> String);
    map.insert("22b".to_string(), day_22b::solve as fn() -> String);
    map.insert("23a".to_string(), day_23a::solve as fn() -> String);
    map.insert("23b".to_string(), day_23b::solve as fn() -> String);
    map.insert("24a".to_string(), day_24a::solve as fn() -> String);
    map.insert("24b".to_string(), day_24b::solve as fn() -> String);
    map.insert("25a".to_string(), day_25a::solve as fn() -> String);
    map.insert("25b".to_string(), day_25b::solve as fn() -> String);
    map
}
