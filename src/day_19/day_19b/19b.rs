// Implementation for Day 19, Part B
use crate::common::get_input;
use std::path::Path;

#[derive(Clone, PartialEq)]
struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: vec![None; 26],
            is_word: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut ptr = self;
        for char in word.chars() {
            let index = char as usize - 'a' as usize;
            if ptr.children[index].is_none() {
                ptr.children[index] = Some(Box::new(TrieNode::new()));
            }
            ptr = ptr.children[index].as_mut().unwrap();
        }
        ptr.is_word = true;
    }
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut split_iter = input.split(|line| line.is_empty());
    let pattern_string = &split_iter.next().unwrap().to_vec()[0];
    let patterns = pattern_string.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();
    let designs = split_iter.next().unwrap().to_vec();

    let mut trie = TrieNode::new();
    for pattern in patterns {
        trie.insert(&pattern);
    }
    
    let mut ans = 0;

    for design in designs {
        let mut dp = vec![0usize; design.len() + 1];
        for (i, _) in design.chars().enumerate() {
            if i == 0 || dp[i - 1] != 0 {
                let mut ptr = &trie;
                for j in i..design.len() {
                    let index = design.as_bytes()[j] as usize - 'a' as usize;
                    if ptr.children[index] == None {
                        break;
                    }
                    ptr = ptr.children[index].as_ref().unwrap();
                    if ptr.is_word {
                        if i > 0 {
                            dp[j] += dp[i - 1];
                        }
                        else {
                            dp[j] += 1;
                        }
                    }
                }
            }
        }
        ans += dp[design.len() - 1];
    }

    
    return ans.to_string();
}
