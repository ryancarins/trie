pub mod naive_trie;
pub mod trimmed_hash_trie;
pub mod trimmed_vec_trie;
pub mod bit_trie;
pub mod trie;

use std::fs::File;
use std::io::{prelude::*, BufReader};

//Helpers
pub fn get_words_as_vec() -> Vec<String> {
    let mut vec = Vec::new();
    let file = File::open("words.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        vec.push(line.unwrap());
    }
    vec
}

//Helpers
pub fn get_3_letter_perms_as_vec() -> Vec<String> {
    let mut vec = Vec::new();
    let file = File::open("3perms.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        vec.push(line.unwrap());
    }
    vec
}
