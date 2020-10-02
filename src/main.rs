mod tree;

use std::io::{self, BufRead};


fn main() {
    let mut tree = tree::TreeWord::new("FirstGraph".to_string());
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(word) => tree.add_word(&word),
            Err(_) => break
        }
    }
    tree.to_graph();
}
