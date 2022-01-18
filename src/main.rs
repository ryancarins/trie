use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::mem::size_of_val;
use std::time::Instant;
use trie::naive_trie::NaiveTrie;
use trie::trimmed_hash_trie::TrimmedHashTrie;
use trie::trimmed_vec_trie::TrimmedVecTrie;
use trie::vec_trie::VecTrie;

fn main() {
    let mut naive_trie = NaiveTrie::default();
    let mut vec_trie = VecTrie::default();
    let mut trimmed_hash_trie = TrimmedHashTrie::default();
    let mut trimmed_vec_trie = TrimmedVecTrie::default();
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
        trimmed_hash_trie.insert(word);
        trimmed_vec_trie.insert(word);
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

    let now = Instant::now();
    for _ in vector.iter() {
        naive_trie.contains("asdfhjklasdfhjlasdf");
        naive_trie.contains("asdfhjklasdfhjlasdf");
        naive_trie.contains("asdfhjklasdfhjlasdf");
        naive_trie.contains("asdfhjklasdfhjlasdf");
    }
    println!(
        "Naive trie took: {}ms to search for {} words that didn't exist",
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

    let now = Instant::now();
    for _ in vector.iter() {
        vec_trie.contains("asdfhjklasdfhjlasdf");
        vec_trie.contains("asdfhjklasdfhjlasdf");
        vec_trie.contains("asdfhjklasdfhjlasdf");
        vec_trie.contains("asdfhjklasdfhjlasdf");
    }
    println!(
        "Vec trie took: {}ms to search for {} words that didn't exist",
        now.elapsed().as_millis(),
        vector.len() * 4
    );

    //Benchmark trimmed trie
    let now = Instant::now();
    for word in vector.iter() {
        trimmed_hash_trie.contains(word);
        trimmed_hash_trie.contains(word);
        trimmed_hash_trie.contains(word);
        trimmed_hash_trie.contains(word);
    }
    println!(
        "Trimmed hash trie took: {}ms to search for {} words that existed",
        now.elapsed().as_millis(),
        vector.len() * 4
    );

    let now = Instant::now();
    for _ in vector.iter() {
        trimmed_hash_trie.contains("asdfhjklasdfhjlasdf");
        trimmed_hash_trie.contains("asdfhjklasdfhjlasdf");
        trimmed_hash_trie.contains("asdfhjklasdfhjlasdf");
        trimmed_hash_trie.contains("asdfhjklasdfhjlasdf");
    }
    println!(
        "Trimmed hash trie took: {}ms to search for {} words that didn't exist",
        now.elapsed().as_millis(),
        vector.len() * 4
    );

    let now = Instant::now();
    for word in vector.iter() {
        trimmed_vec_trie.contains(word);
        trimmed_vec_trie.contains(word);
        trimmed_vec_trie.contains(word);
        trimmed_vec_trie.contains(word);
    }
    println!(
        "Trimmed vec trie took: {}ms to search for {} words that existed",
        now.elapsed().as_millis(),
        vector.len() * 4
    );

    let now = Instant::now();
    for _ in vector.iter() {
        trimmed_vec_trie.contains("asdfhjklasdfhjlasdf");
        trimmed_vec_trie.contains("asdfhjklasdfhjlasdf");
        trimmed_vec_trie.contains("asdfhjklasdfhjlasdf");
        trimmed_vec_trie.contains("asdfhjklasdfhjlasdf");
    }
    println!(
        "Trimmed vec trie took: {}ms to search for {} words that didn't exist",
        now.elapsed().as_millis(),
        vector.len() * 4
    );

    //Benchmark a vector for a baseline O(n) solution
    let now = Instant::now();
    for i in 0..vector.len() / 8 {
        vector.contains(&vector[i]);
    }
    println!(
        "Vector took: {}ms to search for {} words that existed",
        now.elapsed().as_millis(),
        vector.len() / 8
    );

    let string = String::from("asdfhjklasdfhjlasdf");
    let now = Instant::now();
    for i in 0..vector.len() / 8 {
        vector.contains(&string);
    }
    println!(
        "Vector took: {}ms to search for {} words that didn't exist",
        now.elapsed().as_millis(),
        vector.len() / 8
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

    let now = Instant::now();
    for _ in vector.iter() {
        hash_set.contains("asdfhjklasdfhjlasdf");
        hash_set.contains("asdfhjklasdfhjlasdf");
        hash_set.contains("asdfhjklasdfhjlasdf");
        hash_set.contains("asdfhjklasdfhjlasdf");
    }
    println!(
        "HashSet took: {}ms to search for {} words that didn't exist",
        now.elapsed().as_millis(),
        vector.len() * 4
    );
}
