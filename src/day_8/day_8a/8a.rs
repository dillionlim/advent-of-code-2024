// Implementation for Day 8, Part A
use crate::common::{get_input, process_input};
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

fn antinode_locations(node_a: (usize, usize), node_b: (usize, usize)) -> ((isize, isize), (isize, isize)) {
    ((2 * node_a.0 as isize - node_b.0 as isize, 2 * node_a.1 as isize - node_b.1 as isize),
    (2 * node_b.0 as isize - node_a.0 as isize, 2 * node_b.1 as isize - node_a.1 as isize))
}

fn count_antinodes_freq(nodes: &Vec<(usize, usize)>, x_len: usize, y_len: usize, seen: &mut Vec<bool>) -> usize {
    nodes
    .iter()
    .tuple_combinations()
    .map(|(a, b)| {
        let (first_pair, second_pair) = antinode_locations(*a, *b);
        let mut count: usize = 0;
        if in_bounds(first_pair.0, first_pair.1, x_len, y_len) && 
        !seen[first_pair.0 as usize * y_len + first_pair.1 as usize] {
            count += 1;
            seen[first_pair.0 as usize * y_len + first_pair.1 as usize] = true;
        }
        if in_bounds(second_pair.0, second_pair.1, x_len, y_len) && 
        !seen[second_pair.0 as usize * y_len + second_pair.1 as usize] {
            count += 1;
            seen[second_pair.0 as usize * y_len + second_pair.1 as usize] = true;
        }
        count
    })
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
