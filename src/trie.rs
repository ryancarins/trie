pub trait Trie {
    fn insert(&mut self, word: &str);
    fn contains(&mut self, word: &str) -> bool;
    fn delete(&mut self, word: &str) -> bool;
}