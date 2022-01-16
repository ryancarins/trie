mod tests {
    use trie::naive_trie::NaiveTrie;

    #[test]
    fn naive_trie_test_insert() {
        let mut trie = NaiveTrie::default();
        trie.insert("testing");
        assert!(trie.contains("testing"));
        //Make sure that undefined substrings aren't counted as words
        assert!(!trie.contains("test"));
    }

    #[test]
    fn naive_trie_test_delete() {
        let mut trie = NaiveTrie::default();
        trie.insert("testing");
        trie.insert("test");
        assert!(trie.contains("testing"));
        trie.delete("test");
        assert!(!trie.contains("test"));
    }
}
