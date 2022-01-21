mod tests {
    use trie::{
        naive_trie::NaiveTrie, trimmed_hash_trie::TrimmedHashTrie,
        trimmed_vec_trie::TrimmedVecTrie,
    };

    use trie::{get_3_letter_perms_as_vec, get_words_as_vec};

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

    #[test]
    fn trimmed_hash_trie_test_insert() {
        let mut trie = TrimmedHashTrie::default();
        trie.insert("testing");
        assert!(trie.contains("testing"));
        //Make sure that undefined substrings aren't counted as words
        assert!(!trie.contains("test"));
    }

    #[test]
    fn trimmed_hash_trie_test_delete() {
        let mut trie = TrimmedHashTrie::default();
        trie.insert("testing");
        trie.insert("test");
        assert!(trie.contains("testing"));
        trie.delete("test");
        assert!(!trie.contains("test"));
    }

    #[test]
    fn trimmed_vec_trie_test_insert() {
        let mut trie = TrimmedVecTrie::default();
        trie.insert("testing");
        assert!(trie.contains("testing"));
        //Make sure that undefined substrings aren't counted as words
        assert!(!trie.contains("test"));
    }

    #[test]
    fn trimmed_vec_trie_test_delete() {
        let mut trie = TrimmedVecTrie::default();
        trie.insert("testing");
        trie.insert("test");
        assert!(trie.contains("testing"));
        trie.delete("test");
        assert!(!trie.contains("test"));
    }


    //Make sure all words exist
    #[test]
    fn full_check_trimmed_vec() {
        let mut trie = TrimmedVecTrie::default();
        let words = get_words_as_vec();
        let perms = get_3_letter_perms_as_vec();
        for word in &words {
            trie.insert(word);
        }

        for word in &words {
            assert!(trie.contains(word));
        }

        let mut count = 0;
        for perm in &perms {
            if trie.contains(perm) {
                count += 1;
            }
        }
        assert_eq!(count, 2079);
    }

    #[test]
    fn full_check_trimmed_hash() {
        let mut trie = TrimmedHashTrie::default();
        let words = get_words_as_vec();
        let perms = get_3_letter_perms_as_vec();
        for word in &words {
            trie.insert(word);
        }

        for word in &words {
            assert!(trie.contains(word));
        }

        let mut count = 0;
        for perm in &perms {
            if trie.contains(perm) {
                count += 1;
            }
        }
        assert_eq!(count, 2079);
    }

    #[test]
    fn full_check_naive() {
        let mut trie = NaiveTrie::default();
        let words = get_words_as_vec();
        let perms = get_3_letter_perms_as_vec();
        for word in &words {
            trie.insert(word);
        }

        for word in &words {
            assert!(trie.contains(word));
        }

        let mut count = 0;
        for perm in &perms {
            if trie.contains(perm) {
                count += 1;
            }
        }
        assert_eq!(count, 2079);
    }
}
