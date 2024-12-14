// Implementation for Day 14, Part A
use crate::common::get_input;
use std::path::Path;

fn process_input(input: Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    input
    .into_iter()
    .map(|s| {
        let mut parts = s.split_whitespace();
        
        let p = parts.next().expect("Invalid input").strip_prefix("p=").expect("Invalid input").split(',').map(|x| x.parse::<i32>().expect("Invalid input")).collect::<Vec<i32>>();
        let v = parts.next().expect("Invalid input").strip_prefix("v=").expect("Invalid input").split(',').map(|x| x.parse::<i32>().expect("Invalid input")).collect::<Vec<i32>>();

        ((p[0], p[1]), (v[0], v[1]))
    })
    .collect()
}

fn get_final_pos(start: (i32, i32), vel: (i32, i32), time: i32, h: i32, w: i32) -> (i32, i32) {
    (((start.0 + vel.0 * time) % h + h) % h, ((start.1 + vel.1 * time) % w + w) % w)
}

fn classify_quadrant(pos: (i32, i32), h: i32, w: i32) -> Option<i32> {
    let midpt_h = h / 2;
    let midpt_w = w / 2;

    let left = if pos.0 < midpt_h { 0 } else if pos.0 >= (midpt_h + h % 2) { 1 } else { -1 };
    let up = if pos.1 < midpt_w { 0 } else if pos.1 >= (midpt_w + w % 2) { 1 } else { -1 }; 

    if left == -1 || up == -1 {
        return None;
    }
    else {
        return Some(left + up * 2);
    }
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<((i32, i32), (i32, i32))> = process_input(input);

    const TIME: i32 = 100;
    const H: i32 = 101;
    const W: i32 = 103;

    let mut quadrant_totals = (0, 0, 0, 0);

    for &(start, vel) in processed_input.iter() {
        let quadrant = classify_quadrant(get_final_pos(start, vel, TIME, H, W), H, W);
        if quadrant.is_some() {
            let value = quadrant.unwrap();
            match value {
                0 => quadrant_totals.0 += 1,
                1 => quadrant_totals.1 += 1,
                2 => quadrant_totals.2 += 1,
                3 => quadrant_totals.3 += 1,
                _ => {},
            }
        }
    }

    let ans = quadrant_totals.0 * quadrant_totals.1 * quadrant_totals.2 * quadrant_totals.3;
    
    return ans.to_string();
}
