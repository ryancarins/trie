#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use test::Bencher;
    use trie::get_words_as_vec;
    use trie::{
        naive_trie::NaiveTrie, trimmed_hash_trie::TrimmedHashTrie,
        trimmed_vec_trie::TrimmedVecTrie,
    };

    //Insert benchmarks
    #[bench]
    fn bench_insert_naive_trie(b: &mut Bencher) {
        let mut naive_trie = NaiveTrie::default();

        b.iter(|| naive_trie.insert("tester"));
    }

    #[bench]
    fn bench_insert_trimmed_hash_trie(b: &mut Bencher) {
        let mut trimmed_hash_trie = TrimmedHashTrie::default();

        b.iter(|| trimmed_hash_trie.insert("tester"));
    }

    #[bench]
    fn bench_insert_trimmed_vec_trie(b: &mut Bencher) {
        let mut trimmed_vec_trie = TrimmedVecTrie::default();

        b.iter(|| trimmed_vec_trie.insert("tester"));
    }

    //Existing lookup benchmarks
    #[bench]
    fn bench_contains_existing_naive_trie(b: &mut Bencher) {
        let mut naive_trie = NaiveTrie::default();
        let words = get_words_as_vec();
        for word in words {
            naive_trie.insert(&word);
        }

        b.iter(|| {
            let _blank = naive_trie.contains("tester");
        });
    }

    #[bench]
    fn bench_contains_existing_trimmed_hash_trie(b: &mut Bencher) {
        let mut trimmed_hash_trie = TrimmedHashTrie::default();
        let words = get_words_as_vec();
        for word in words {
            trimmed_hash_trie.insert(&word);
        }
        b.iter(|| {
            let _blank = trimmed_hash_trie.contains("tester");
        });
    }

    #[bench]
    fn bench_contains_existing_trimmed_vec_trie(b: &mut Bencher) {
        let mut trimmed_vec_trie = TrimmedVecTrie::default();
        let words = get_words_as_vec();
        for word in words {
            trimmed_vec_trie.insert(&word);
        }
        b.iter(|| {
            let _blank = trimmed_vec_trie.contains("tester");
        });
    }
}
