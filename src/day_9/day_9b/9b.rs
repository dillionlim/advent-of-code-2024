// Implementation for Day 9, Part A
use crate::common::get_input;
use std::path::Path;

#[derive(Debug)]
struct SegmentTree {
    tree: Vec<u8>,
    padding: usize,
}

impl SegmentTree {
    fn new(size: usize, input: Vec<u8>) -> Self {
        let mut seg_tree = SegmentTree {
            tree: vec![0; 4 * size],
            padding: 0
        };

        let padding = 1<<((size as f64).log2().ceil() as usize);
        seg_tree.padding = padding;

        for (i, num) in input.iter().enumerate() {
            seg_tree.tree[padding + i] = *num;
        }
        for i in (1..=(padding - 1)).rev() {
            seg_tree.tree[i] = seg_tree.tree[i << 1].max(seg_tree.tree[i << 1 | 1]);
        }

        seg_tree
    }

    fn find_leftmost_geq_value(&self, value: u8) -> Option<usize> {
        // all the numbers in the array are smaller than value
        if self.tree[1] < value { 
            return None;
        }

        let mut ind = 1;
        
        while ind < self.padding {
            let leftmax = self.tree[ind << 1];
            let rightmax = self.tree[ind << 1 | 1];
            if leftmax >= value {
                ind = ind << 1;
            }
            else if rightmax >= value {
                ind = ind << 1 | 1;
            }
        }

        Some(ind - self.padding)
    }

    fn update(&mut self, index: usize, value: u8) {
        let mut ind = self.padding as usize + index;
        self.tree[ind] -= value;
        while ind > 1 {
            ind >>= 1;
            self.tree[ind] = self.tree[ind << 1].max(self.tree[ind << 1 | 1]);
        }
    }
}

fn process_input(input: &String) -> (Vec<u8>, Vec<u8>, Vec<u32>) {
    let mut blocks: Vec<u8> = Vec::new();
    let mut space: Vec<u8> = Vec::new();
    let mut prefix_position: Vec<u32> = Vec::new();
    prefix_position.push(0);
    
    for (i, c) in input.chars().enumerate() {
        let num = c.to_digit(10).unwrap() as u8;
        if i % 2 == 0 {
            blocks.push(num);
            prefix_position.push(prefix_position.last().unwrap() + num as u32);
        }
        else {
            // if the previous memory block was empty, we merge contiguous spaces
            if *blocks.last().unwrap() == 0 {
                blocks.pop();
                let last_space = space.pop().unwrap_or(0);
                space.push(num + last_space);
                prefix_position.pop();
                let last_prefix = prefix_position.pop().unwrap_or(0);
                prefix_position.push(num as u32 + last_prefix);
            }
            else { 
                space.push(num);
                prefix_position.push(prefix_position.last().unwrap() + num as u32);
            }
        }
    }

    (blocks, space, prefix_position)
}

fn sum(start: u32, end: u32) -> u64 {
    // not inclusive of end
    ((start + end - 1) * (end - start) / 2) as u64
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);

    // if the length is even, the last empty space is of no use to us, pop it
    let (memory_blocks, space_blocks, mut prefix_position) = process_input(&input[0]);

    let mut checksum: u64 = 0;

    let mut seg_tree = SegmentTree::new(space_blocks.len(), space_blocks);

    for (id, block) in memory_blocks.iter().enumerate().rev() {
        let first_occurence = seg_tree.find_leftmost_geq_value(*block);
        match first_occurence {
            Some(index) => { // memory block was found starting at index to replace the block, shift it there
                // reduce the number of spaces at that index by block
                if index < id {
                    seg_tree.update(index, *block);
                    checksum += sum(prefix_position[index * 2 + 1], prefix_position[index * 2 + 1] as u32 + *block as u32) * id as u64;
                    prefix_position[index * 2 + 1] += *block as u32;
                }
                else {
                    // this will be shifting it left, so don't shift it
                    checksum += sum(prefix_position[id * 2], prefix_position[id * 2] + *block as u32) * id as u64;
                }
            }
            None => { // no memory block was found to replace the block, do not shift
                checksum += sum(prefix_position[id * 2], prefix_position[id * 2] + *block as u32) * id as u64;
            }
        }
    }

    return checksum.to_string();
}
