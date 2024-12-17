// Implementation for Day 17, Part A
use crate::common::get_input;
use std::path::Path;

fn process_input(input: &Vec<String>) -> (u32, u32, u32, Vec<u8>) {
    let mut registers = (0, 0, 0);
    let mut program = Vec::new();
    for line in input {
        if line.starts_with("Register A:") {
            registers.0 = line
            .split_once(":")
            .unwrap().1
            .trim()
            .parse()
            .unwrap(); 
        } 
        else if line.starts_with("Register B:") {
            registers.1 = line
            .split_once(":")
            .unwrap().1
            .trim()
            .parse()
            .unwrap(); 
        } 
        else if line.starts_with("Register C:") {
            registers.2 = line
            .split_once(":")
            .unwrap().1
            .trim()
            .parse()
            .unwrap(); 
        } 
        else if line.starts_with("Program:") {
            program = line
            .split_once(":")
            .unwrap().1
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect(); 
        }
    }

    (registers.0, registers.1, registers.2, program)
}

fn get_combo_operand_val (operand: u8, a: u32, b: u32, c: u32) -> u32 {
    match operand {
        0..=3 => operand as u32,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid operand"),
    }
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let (mut a, mut b, mut c, program): (u32, u32, u32, Vec<u8>) = process_input(&input);
    
    let mut instruction_ptr = 0;

    let mut ans="".to_string();

    while instruction_ptr < program.len() {
        let (opcode, operand) = (program[instruction_ptr], program[instruction_ptr + 1]);
        match opcode {
            0 => {
                let value = get_combo_operand_val(operand, a, b, c);
                a /= 1 << value;
            },
            1 => {
                b ^= operand as u32;
            },
            2 => {
                let value = get_combo_operand_val(operand, a, b, c);
                b = value % 8;
            },
            3 => {
                if a != 0 {
                    instruction_ptr = operand as usize;
                    continue;
                }
            },
            4 => {
                b ^= c;
            }, 
            5 => {
                ans.push_str(&(get_combo_operand_val(operand, a, b, c) % 8).to_string());
                ans.push_str(",");
            },
            6 => {
                let value = get_combo_operand_val(operand, a, b, c);
                b = a/(1 << value);
            }, 
            7 => {
                let value = get_combo_operand_val(operand, a, b, c);
                c = a/(1 << value);
            }
            _ => panic!("Invalid opcode"),
        }
        instruction_ptr += 2;
    }

    ans.pop();

    return ans.to_string();
}
