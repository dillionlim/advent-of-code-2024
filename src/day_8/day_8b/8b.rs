// Implementation for Day 8, Part A
use crate::common::{get_input, process_input};
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn antinode_locations_count(node_a: (usize, usize), node_b: (usize, usize), x_len: usize, y_len: usize, seen: &mut Vec<bool>) -> usize {
    let (x1, y1) = (node_a.0 as isize, node_a.1 as isize);
    let (x2, y2) = (node_b.0 as isize, node_b.1 as isize);

    let unit_vec = gcd(x2 - x1, y2 - y1);
    let dx_hat = (x2-x1)/unit_vec;
    let dy_hat = (y2-y1)/unit_vec;

    let mut collinear_point_count = 0;

    let mut x = x1;
    let mut y = y1;

    while in_bounds(x, y, x_len, y_len) {
        if !seen[x as usize * y_len + y as usize] {
            collinear_point_count += 1;
            seen[x as usize * y_len + y as usize] = true;
        }
        x -= dx_hat;
        y -= dy_hat;
    }

    x = x1;
    y = y1;

    while in_bounds(x, y, x_len, y_len) {
        if !seen[x as usize * y_len + y as usize] {
            collinear_point_count += 1;
            seen[x as usize * y_len + y as usize] = true;
        }
        x += dx_hat;
        y += dy_hat;
    }

    collinear_point_count
}

fn count_antinodes_freq(nodes: &Vec<(usize, usize)>, x_len: usize, y_len: usize, seen: &mut Vec<bool>) -> usize {
    nodes
    .iter()
    .tuple_combinations()
    .map(|(a, b)| antinode_locations_count(*a, *b, x_len, y_len, seen))
    .sum()
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<Vec<char>> = process_input(input, "char");

    let mut seen: Vec<bool> = vec![false; processed_input.len() * processed_input[0].len()];

    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    
    processed_input
    .iter()
    .enumerate()
    .for_each(|(i, row)| {
        row.iter()
            .enumerate()
            .for_each(|(j, freq)| {
                if *freq!= '.' {
                    nodes
                    .entry(freq.clone())
                    .or_insert(Vec::new())
                    .push((i, j))
                }
            })
    });

    let mut total_antinodes = 0;

    for (_, nodes) in nodes.iter() {
        total_antinodes += count_antinodes_freq(nodes, processed_input.len(), processed_input[0].len(), &mut seen);
    }

    return total_antinodes.to_string();
}
