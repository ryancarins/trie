mod tests {
    use trie::Trie;

    #[test]
    fn test_insert() {
        let mut trie = Trie::default();
        trie.insert("testing");
        assert!(trie.contains("testing"));
        //Make sure that undefined substrings aren't counted as words
        assert!(!trie.contains("test"));
    }
}
