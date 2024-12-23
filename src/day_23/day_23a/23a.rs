// Implementation for Day 23, Part A
use crate::common::get_input;
use std::path::Path;

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let parsed_values = input.iter().map(|s| {
        let first_hash = (s.as_bytes()[0] - b'a') as u32 * 26 + (s.as_bytes()[1] - b'a') as u32;
        let second_hash = (s.as_bytes()[3] - b'a') as u32 * 26 + (s.as_bytes()[4] - b'a') as u32;
        (first_hash, second_hash)
    }).collect::<Vec<(u32, u32)>>();

    let mut adj = vec![vec![]; 676];
    let mut edges = vec![vec![false; 676]; 676];

    for (edge_from, edge_to) in parsed_values {
        adj[edge_from as usize].push(edge_to);
        adj[edge_to as usize].push(edge_from);
        edges[edge_from as usize][edge_to as usize] = true;
        edges[edge_to as usize][edge_from as usize] = true;
    }

    let mut vis = vec![false; 676];
    let mut ans = 0;

    for node in 494..520 { // let the first node start with t, so we skip the filtering
        let neighbours = &adj[node as usize];
        if neighbours.len() > 0 {
            vis[node] = true;
            for (i, &neighbour1) in neighbours.iter().enumerate() {
                for &neighbour2 in neighbours.iter().skip(i) {
                    if !vis[neighbour1 as usize] && !vis[neighbour2 as usize] 
                    && edges[neighbour1 as usize][neighbour2 as usize] {
                        ans += 1;
                    }
                }
            }
        }
    }

    return ans.to_string();
}
