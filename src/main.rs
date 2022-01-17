use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;
use trie::naive_trie::NaiveTrie;
use trie::trimmed_hash_trie::TrimmedHashTrie;
use trie::vec_trie::VecTrie;

fn main() {
    let mut naive_trie = NaiveTrie::default();
    let mut vec_trie = VecTrie::default();
    let mut trimmed_trie = TrimmedHashTrie::default();
    let mut hash_set: HashSet<String> = HashSet::new();

    //Load words into a vector so we can easily insert into multiple kinds of trie
    let mut vector = Vec::new();
    let file = File::open("words.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        vector.push(line.unwrap());
    }

    for word in vector.iter() {
        naive_trie.insert(word);
        vec_trie.insert(word);
        trimmed_trie.insert(word);
        hash_set.insert(String::from(word));
    }

    //Benchmark naive trie
    let now = Instant::now();
    for word in vector.iter() {
        naive_trie.contains(word);
        naive_trie.contains(word);
        naive_trie.contains(word);
        naive_trie.contains(word);
    }
    println!(
        "Naive trie took: {}ms to search for {} words that existed",
        now.elapsed().as_millis(),
        vector.len() * 4
    );

    //Benchmark vec trie
    let now = Instant::now();
    for word in vector.iter() {
        vec_trie.contains(word);
        vec_trie.contains(word);
        vec_trie.contains(word);
        vec_trie.contains(word);
    }
    println!(
        "Vec trie took: {}ms to search for {} words that existed",
        now.elapsed().as_millis(),
        vector.len() * 4
    );

    //Benchmark trimmed trie
    let now = Instant::now();
    for word in vector.iter() {
        trimmed_trie.contains(word);
        trimmed_trie.contains(word);
        trimmed_trie.contains(word);
        trimmed_trie.contains(word);
    }
    println!(
        "Trimmed hash trie took: {}ms to search for {} words that existed",
        now.elapsed().as_millis(),
        vector.len() * 4
    );

    //Benchmark a vector for a baseline O(n) solution
    let now = Instant::now();
    for word in vector.iter() {
        vector.contains(word);
        vector.contains(word);
        vector.contains(word);
        vector.contains(word);
    }
    println!(
        "Vector took: {}ms to search for {} words that existed",
        now.elapsed().as_millis(),
        vector.len() * 4
    );

    //Benchmark a HashSet for a fair test
    let now = Instant::now();
    for word in vector.iter() {
        hash_set.contains(word);
        hash_set.contains(word);
        hash_set.contains(word);
        hash_set.contains(word);
    }
    println!(
        "HashSet took: {}ms to search for {} words that existed",
        now.elapsed().as_millis(),
        vector.len() * 4
    );
}
