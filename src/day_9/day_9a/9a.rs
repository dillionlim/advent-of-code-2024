// Implementation for Day 9, Part A
use crate::common::get_input;
use std::path::Path;

fn process_input(input: &String) -> Vec<u8> {
    let mut digits: Vec<u8> = input
        .chars() 
        .map(|c| c.to_digit(10).expect("Input must contain only numeric characters") as u8) // Convert each character to u8
        .collect();
    
    if input.len() % 2 == 0 {
        digits.pop();
    }

    digits
}

fn sum(start: u32, end: u32) -> u64 {
    // not inclusive of end
    ((start + end - 1) * (end - start) / 2) as u64
}

fn get_id_from_pos(pos: usize) -> u64 {
    (pos / 2) as u64
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);

    // if the length is even, the last empty space is of no use to us, pop it
    let mut processed_input: Vec<u8> = process_input(&input[0]);

    let mut checksum: u64 = 0;

    let mut l_ptr: usize = 0;
    let mut r_ptr: usize = processed_input.len() - 1;
    let mut curr_ind: u32 = 0;

    while l_ptr <= r_ptr {
        let block_empty: bool = l_ptr % 2 != 0;
        if block_empty { 
            let mut no_of_spaces = processed_input[l_ptr];
            while l_ptr <= r_ptr && no_of_spaces > 0 {
                let no_of_digits = processed_input[r_ptr];
                if no_of_digits <= no_of_spaces { // can fit all in this space
                    no_of_spaces -= no_of_digits;
                    let next_ind = curr_ind + no_of_digits as u32;
                    processed_input[r_ptr] = 0;
                    checksum += sum(curr_ind, next_ind) * get_id_from_pos(r_ptr);
                    curr_ind = next_ind;

                    // skip the spaces to get to the next filled block
                    r_ptr -= 2;
                }
                else { // cannot fill all in this space, fill as much as possible and reduce accordingly
                    let next_ind = curr_ind + no_of_spaces as u32;
                    processed_input[r_ptr] -= no_of_spaces;
                    no_of_spaces = 0;
                    checksum += sum(curr_ind, next_ind) * get_id_from_pos(r_ptr);
                    curr_ind = next_ind;
                }
            }
        }
        else {
            let next_ind = curr_ind + processed_input[l_ptr] as u32;
            checksum += sum(curr_ind, next_ind) * get_id_from_pos(l_ptr);
            curr_ind = next_ind;
        }

        l_ptr += 1;
    }
    
    return checksum.to_string();
}
