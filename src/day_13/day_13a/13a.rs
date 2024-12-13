// Implementation for Day 13, Part A
use crate::common::get_input;
use std::path::Path;

fn process_input(input: Vec<String>) -> Vec<((i32, i32), (i32, i32), (i32, i32))> {
    let mut result = Vec::new();
    let mut current_tuple = ((0, 0), (0, 0), (0, 0));

    for line in input {
        if line.starts_with("Button A:") {
            let parts: Vec<&str> = line["Button A:".len()..].split(",").map(|s| s.trim()).collect();
            let x = parts[0]["X+".len()..].parse::<i32>().unwrap();
            let y = parts[1]["Y+".len()..].parse::<i32>().unwrap();
            current_tuple.0 = (x, y);
        } 
        else if line.starts_with("Button B:") {
            let parts: Vec<&str> = line["Button B:".len()..].split(",").map(|s| s.trim()).collect();
            let x = parts[0]["X+".len()..].parse::<i32>().unwrap();
            let y = parts[1]["Y+".len()..].parse::<i32>().unwrap();
            current_tuple.1 = (x, y);
        } 
        else if line.starts_with("Prize:") {
            let parts: Vec<&str> = line["Prize:".len()..].split(",").map(|s| s.trim()).collect();
            let x = parts[0]["X=".len()..].parse::<i32>().unwrap();
            let y = parts[1]["Y=".len()..].parse::<i32>().unwrap();
            current_tuple.2 = (x, y);
            result.push(current_tuple);
        }
    }

    result
}

fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = extended_euclidean(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;
    (g, x, y)
}  

fn process_testcase(button_a: (i32, i32), button_b: (i32, i32), prize: (i32, i32)) -> Option<i32> {
    // find determinant of matrix comprising button_a and button_b

    let det = button_a.0 * button_b.1 - button_a.1 * button_b.0;

    if det != 0 { // non-singular matrix, check if integer solution exists
        let a_numerator = button_b.1 * prize.0 - button_b.0 * prize.1;
        let b_numerator = button_a.0 * prize.1 - button_a.1 * prize.0;

        if a_numerator % det == 0 && b_numerator % det == 0 {
            let a_presses = a_numerator / det;
            let b_presses = b_numerator / det;
            if a_presses > 0 && b_presses > 0 {
                return Some(a_presses * 3 + b_presses);
            }
        }
    }
    else { // singular matrix
        // check if prize is in the integer column space of matrix

        if button_a.0 * prize.1 == button_a.1 * prize.0 {
            // infinitely many solutions
            
            // use extended gcd to find min x and y
            // for the equation ax + by = c

            let (g, a_press, b_press) = extended_euclidean(button_a.0, button_b.0);

            // ensure the gcd divides the constant c
            if prize.0 % g != 0 {
                return None;
            }

            // scale back
            let scale = prize.0 / g;
            let a_presses = a_press * scale;
            let b_presses = b_press * scale;

            let k0 = if button_b.0 != 0 { -(a_presses / (button_b.0 / g)) } else { 0 };
        
            // first minimal pair
            let a_min_1 = a_presses + k0 * (button_b.0 / g);
            let b_min_1 = b_presses - k0 * (button_a.0 / g);

            if a_min_1 > 0 && b_min_1 > 0 {
                return Some(a_min_1 * 3 + b_min_1);
            }
        
            // otherwise, second minimal pair
            let k1 = k0 + 1;
            let a_min_2 = a_presses + k1 * (button_b.0 / g);
            let b_min_2 = b_presses - k1 * (button_a.0 / g);

            if a_min_2 > 0 && b_min_2 > 0 {
                return Some(a_min_2 * 3 + b_min_2);
            }
        }

        // otherwise no solution
    }
    None
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input = process_input(input);

    let mut ans = 0;

    for (button_a, button_b, prize) in processed_input {
        if let Some(result) = process_testcase(button_a, button_b, prize) {
            ans += result;
        }
    }
    
    return ans.to_string();
}
