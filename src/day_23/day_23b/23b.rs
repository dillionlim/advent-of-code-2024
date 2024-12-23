// Implementation for Day 23, Part B
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
    let mut clique = Vec::new();
    let mut largest_clique = Vec::new();

    for (node, neighbours) in adj.iter().enumerate() {
        if !vis[node] {
            clique.clear();
            clique.push(node as u32);
            for &neighbour in neighbours {
                if clique.iter().all(|&other| edges[other as usize][neighbour as usize]) {
                    vis[neighbour as usize] = true; // optimization, since this node will give the same clique
                    clique.push(neighbour);
                }
            }
            if clique.len() > largest_clique.len() {
                largest_clique = clique.clone();
            }
        }
    }

    let mut ans = String::new();
    largest_clique.sort();

    for hashed_key in largest_clique {
        let first_letter = ((hashed_key / 26) as u8 + b'a') as char;
        let second_letter = ((hashed_key % 26) as u8 + b'a') as char;
        ans.push(first_letter);
        ans.push(second_letter);
        ans.push(',');
    }

    ans.pop();

    return ans;
}
