mod tests {
    use trie::{
        naive_trie::NaiveTrie, trimmed_hash_trie::TrimmedHashTrie,
        trimmed_vec_trie::TrimmedVecTrie, vec_trie::VecTrie,
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

    #[test]
    fn vec_trie_test_insert() {
        let mut trie = VecTrie::default();
        trie.insert("test");
        assert!(trie.contains("test"));
        //Make sure that undefined substrings aren't counted as words
        assert!(!trie.contains("tes"));
    }

    #[test]
    fn vec_trie_test_delete() {
        let mut trie = VecTrie::default();
        trie.insert("test");
        trie.insert("te");
        assert!(trie.contains("test"));
        trie.delete("te");
        assert!(!trie.contains("te"));
    }

    //Node counting
    #[test]
    fn count_vec_trie_nodes() {
        let mut trie = VecTrie::default();
        trie.insert("zzzzz");
        trie.insert("zzzz");
        trie.insert("zzz");
        trie.insert("zzza");
        trie.insert("zzzab");
        trie.insert("zzzac");
        trie.insert("zz");
        trie.insert("z");
        assert_eq!(trie.count_nodes(), 8);
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
    fn full_check_vec_trie() {
        let mut trie = VecTrie::default();
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
