// Implementation for Day 24, Part B
use crate::common::get_input;
use std::path::Path;
use hashbrown::{HashSet, HashMap};

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut split_iter = input.split(|line| line.is_empty());
    let _ = split_iter.next();
    let input_connections = split_iter.next().unwrap().to_vec();

    // operator, (first dependency, second dependency, final value)
    let mut xor_half_adder_input_list: Vec<(String, String, String)> = Vec::new();
    let mut xor_half_adder_carry_list: HashMap<String, (String, String)> = HashMap::new();
    let mut output_list: Vec<(String, String)> = Vec::new();
    let mut exists_in_carry_list: HashSet<String> = HashSet::new();
    let mut or_output: HashSet<String> = HashSet::new();

    let mut invalid_codes: HashSet<String> = HashSet::new();

    for line in input_connections {
        let (dependencies, code) = line.split_once(" -> ").unwrap();
        let mut dependency_iter = dependencies.split_whitespace();
        let first_dependency = dependency_iter.next().unwrap().trim().to_string();
        let operator = dependency_iter.next().unwrap().trim().to_string();
        let second_dependency = dependency_iter.next().unwrap().trim().to_string();
        
        if operator == "XOR" {
            if first_dependency.as_bytes()[0] == b'x' || second_dependency.as_bytes()[0] == b'x' {
                xor_half_adder_input_list.push((first_dependency.clone(), second_dependency.clone(), code.trim().to_string()));
            }
            else {
                // these should be of the form intermediate XOR carryIn -> sum (so should be zZZ)
                // should be of the form xXX XOR yYY -> zZZ
                let code = code.trim().to_string();
                if code.as_bytes()[0] != b'z' { // these must be leading to outputs
                    invalid_codes.insert(code.clone());
                }
                xor_half_adder_carry_list.insert(code, (first_dependency.clone(), second_dependency.clone()));
                exists_in_carry_list.insert(first_dependency.clone());
                exists_in_carry_list.insert(second_dependency.clone());
            }
        }
        else if operator == "OR" {
            or_output.insert(code.trim().to_string());
        }
        if code.trim().as_bytes()[0] == b'z' { 
            output_list.push((operator, code.trim().to_string()));
        }
    }

    // these should be of the form xXX XOR yYY -> intermediateXOR (so should not be zZZ)
    // except for x00 XOR y00 -> z00 (since first adder is a half adder)
    for (first_dependency, second_dependency, output) in xor_half_adder_input_list.clone() {
        if first_dependency == "x00" || second_dependency == "x00" {
            if output != "z00"{
                invalid_codes.insert(output.clone()); // first half adder
            }
            continue;
        }
        else if output == "z00" { // invalid half adder
            invalid_codes.insert(output.clone());
        }

        if output.as_bytes()[0] == b'z' { // these should not be leading to outputs
            invalid_codes.insert(output.clone());
        }
    }

    // check all output gates
    // they should be of the form xXX XOR yYY -> zZZ
    // except for the last one, which should be of the form intermediate OR intermediate -> zZZ

    let last_output = format!("z{:02}", output_list.len() - 1);
    for (operator, output) in output_list {
        if output == last_output {
            if operator != "OR" {
                invalid_codes.insert(output.clone());
            }
            continue;
        }
        else {
            if operator != "XOR" {
                invalid_codes.insert(output.clone());
            }
        }
    }

    // for each output of the input set, it should output to a corresponding carry gate

    for (first_dependency, _, output) in xor_half_adder_input_list {
        if invalid_codes.contains(&output) {
            continue;
        }
        if output == "z00" {
            continue;
        }
        if !exists_in_carry_list.contains(&output) {
            invalid_codes.insert(output.clone());

            // ok, so this output gate should output to a carry gate... 
            // given inputs xXX XOR yXX it should output to zXX...

            let output_carry_gate = if first_dependency.len() >= 3 {
                format!("z{:02}", &first_dependency[1..=2])
            }
            else {
                panic!("Invalid first dependency: {}", first_dependency);
            };

            if !invalid_codes.contains(&output_carry_gate) {
                let (actual_first_dependency, actual_second_dependency) = xor_half_adder_carry_list.get(&output_carry_gate).unwrap();

                // so, recall that our xXX XOR yXX -> actually gives a wrong carry gate
                // we need to find the correct carry gate then (because that is also in the wrong position)

                // ok, so this is an xor which is going to a zXX gate. 
                // so, one of the inputs must be a carry input from an OR gate
                // it must have come from an OR gate then, this is not the wrong one
                // the other one must be the wrong carry gate then 

                if or_output.contains(actual_first_dependency) { // this one is from the or gate, other is wrong
                    invalid_codes.insert(actual_second_dependency.clone());
                }
                else {
                    invalid_codes.insert(actual_first_dependency.clone());
                }
            }
        }
    }

    if invalid_codes.len() != 8 {
        panic!("Not all codes have been found / too many codes! Codes found: {:?}", invalid_codes);
    }

    let mut invalid_codes_ans = Vec::from_iter(invalid_codes);
    invalid_codes_ans.sort();
    let ans = invalid_codes_ans.join(",");

    return ans;
}
