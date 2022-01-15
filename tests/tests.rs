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

    #[test]
    fn test_delete() {
        let mut trie = Trie::default();
        trie.insert("testing");
        trie.insert("test");
        assert!(trie.contains("testing"));
        trie.delete("test");
        assert!(!trie.contains("test"));
    }
}
