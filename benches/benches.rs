#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use std::collections::{BTreeSet, HashSet};
    use test::Bencher;
    use trie::get_words_as_vec;
    use trie::trie::Trie;
    use trie::{
        naive_trie::NaiveTrie, trimmed_hash_trie::TrimmedHashTrie, trimmed_vec_trie::TrimmedVecTrie,
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

    #[bench]
    fn bench_insert_hashset(b: &mut Bencher) {
        let mut hashset = HashSet::new();
        let string = String::from("tester");

        b.iter(|| hashset.insert(&string));
    }

    #[bench]
    fn bench_insert_btreeset(b: &mut Bencher) {
        let mut btree = BTreeSet::new();
        let string = String::from("tester");

        b.iter(|| btree.insert(&string));
    }

    #[bench]
    fn bench_insert_vec(b: &mut Bencher) {
        let mut vec = Vec::new();
        let string = String::from("tester");

        b.iter(|| vec.push(&string));
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

    #[bench]
    fn bench_contains_hashset(b: &mut Bencher) {
        let mut hashset = HashSet::new();
        let words = get_words_as_vec();
        for word in words {
            hashset.insert(word);
        }
        b.iter(|| {
            let _blank = hashset.contains("tester");
        });
    }

    #[bench]
    fn bench_contains_btreeset(b: &mut Bencher) {
        let mut btree = BTreeSet::new();
        let words = get_words_as_vec();
        for word in words {
            btree.insert(word);
        }
        b.iter(|| {
            let _blank = btree.contains("tester");
        });
    }

    #[bench]
    fn bench_contains_vec(b: &mut Bencher) {
        let vec = get_words_as_vec();
        let string = String::from("tester");
        b.iter(|| {
            let _blank = vec.contains(&string);
        });
    }

    #[bench]
    fn bench_contains_sorted_vec(b: &mut Bencher) {
        let mut vec = get_words_as_vec();
        vec.sort();
        let string = String::from("tester");
        b.iter(|| {
            let _blank = vec.binary_search(&string);
        });
    }
}
