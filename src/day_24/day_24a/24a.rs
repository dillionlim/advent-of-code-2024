// Implementation for Day 24, Part A
use crate::common::get_input;
use std::path::Path;
use hashbrown::HashMap;

fn rec_solve(node_vals: &mut HashMap<String, bool>, op_list: &HashMap<String, (String, String, String)>, code: &str) -> bool {
    if node_vals.contains_key(code) {
        return *node_vals.get(code).unwrap();
    }
    let (first_dependency, second_dependency, operator) = op_list.get(code).unwrap();
    let value_first = rec_solve(node_vals, op_list, first_dependency);
    let value_second = rec_solve(node_vals, op_list, second_dependency);
    let value = match operator.as_str() {
        "AND" => value_first & value_second,
        "OR" => value_first | value_second,
        "XOR" => value_first ^ value_second,
        _ => panic!("Invalid operator: {}", operator),
    };
    node_vals.insert(code.to_string(), value);
    value
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut split_iter = input.split(|line| line.is_empty());
    let input_initials = split_iter.next().unwrap().to_vec();
    let input_connections = split_iter.next().unwrap().to_vec();

    let mut node_vals: HashMap<String, bool> = HashMap::new();
    input_initials
    .iter()
    .for_each(|line| {
        let (code, initial_value) = line.split_once(':').unwrap();
        let initial_value = initial_value.trim().parse::<u8>().unwrap();
        node_vals.insert(code.to_string(), (initial_value != 0) as bool);
    });

    // ending code, (first dependency, second dependency, operator)
    let mut op_list: HashMap<String, (String, String, String)> = HashMap::new();
    let mut z_count = 0;

    for line in input_connections {
        let (dependencies, code) = line.split_once(" -> ").unwrap();
        let mut dependency_iter = dependencies.split_whitespace();
        let first_dependency = dependency_iter.next().unwrap().trim().to_string();
        let operator = dependency_iter.next().unwrap().trim().to_string();
        let second_dependency = dependency_iter.next().unwrap().trim().to_string();

        if code.as_bytes()[0] == b'z' {
            z_count += 1;
        }
        
        op_list.insert(code.trim().to_string(), (first_dependency, second_dependency, operator));
    }

    let mut ans : u64 = 0;

    for i in (0..z_count).rev() {
        ans <<= 1;
        let start_code = format!("z{:02}", i);
        let result = rec_solve(&mut node_vals, &op_list, &start_code);
        ans |= if result { 1 } else { 0 };
    }
    
    return ans.to_string();
}
