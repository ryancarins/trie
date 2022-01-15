use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::{thread, time};
use trie::Trie;
fn main() {
    let mut trie = Trie::default();

    let file = File::open("words.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        trie.insert(&line.unwrap());
    }
}
