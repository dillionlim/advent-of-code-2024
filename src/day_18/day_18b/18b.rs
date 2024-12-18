// Implementation for Day 18, Part B
use crate::common::get_input;
use std::path::Path;

const DIRS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(x: isize, y: isize, x_len: usize, y_len: usize) -> bool {
    x >= 0 && x < x_len as isize && y >= 0 && y < y_len as isize
}

pub struct DSU {
    parent: Vec<i32>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        DSU {
            parent: vec![-1; n],
        }
    }
    
    pub fn get(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            x
        } 
        else {
            self.parent[x] = self.get(self.parent[x] as usize) as i32;
            self.parent[x] as usize
        }
    }
    
    pub fn same_set(&mut self, a: usize, b: usize) -> bool {
        self.get(a) == self.get(b)
    }
    
    pub fn unite(&mut self, x: usize, y: usize) {
        let mut x_root = self.get(x);
        let mut y_root = self.get(y);

        if x_root == y_root {
            return;
        }
        
        if self.parent[x_root] > self.parent[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        self.parent[x_root] += self.parent[y_root];
        self.parent[y_root] = x_root as i32;
    }
}

fn process_input(input: Vec<String>) -> Vec<(usize, usize)> {
    input
    .into_iter()
    .map(|line| {
        let split_vals = line
        .split_once(',')
        .unwrap();
        
        (split_vals.0.parse::<usize>().unwrap(), split_vals.1.parse::<usize>().unwrap())
    })
    .collect()
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let processed_input: Vec<(usize, usize)> = process_input(input);

    let mut fallen = vec![false; 71 * 71];
    for (x, y) in processed_input.iter() {
        fallen[*x * 71 + *y] = true;
    }

    let mut dsu = DSU::new(71 * 71);

    for i in 0..71 {
        for j in 0..71 {
            if i > 0 && !fallen[(i - 1) * 71 + j] {
                dsu.unite((i - 1) * 71 + j, i * 71 + j);
            }
            if j > 0 && !fallen[i * 71 + (j - 1)] {
                dsu.unite(i * 71 + (j - 1), i * 71 + j);
            }
        }
    }

    for (x, y) in processed_input.iter().rev() {
        let curr_ind = *x * 71 + *y;
        for (dx, dy) in DIRS.iter() {
            let newx = *x as isize + dx;
            let newy = *y as isize + dy;
            if in_bounds(newx, newy, 71, 71) {
                dsu.unite(curr_ind, newx as usize * 71 + newy as usize);
            }
        }
        if dsu.same_set(0, 70 * 71 + 70) {
            return format!("{},{}", x, y);
        }
    }
    
    return "0".to_string();
}
