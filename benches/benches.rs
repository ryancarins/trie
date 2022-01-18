#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use test::Bencher;
    use trie::{
        naive_trie::NaiveTrie, trimmed_hash_trie::TrimmedHashTrie,
        trimmed_vec_trie::TrimmedVecTrie, vec_trie::VecTrie,
    };

    //Insert benchmarks
    #[bench]
    fn bench_insert_naive_trie(b: &mut Bencher) {
        let mut naive_trie = NaiveTrie::default();

        b.iter(|| naive_trie.insert("test"));
    }

    #[bench]
    fn bench_insert_vec_trie(b: &mut Bencher) {
        let mut vec_trie = VecTrie::default();

        b.iter(|| vec_trie.insert("test"));
    }

    #[bench]
    fn bench_insert_trimmed_hash_trie(b: &mut Bencher) {
        let mut trimmed_hash_trie = TrimmedHashTrie::default();

        b.iter(|| trimmed_hash_trie.insert("test"));
    }

    #[bench]
    fn bench_insert_trimmed_vec_trie(b: &mut Bencher) {
        let mut trimmed_vec_trie = TrimmedVecTrie::default();

        b.iter(|| trimmed_vec_trie.insert("test"));
    }

    //Existing lookup benchmarks
    #[bench]
    fn bench_contains_existing_naive_trie(b: &mut Bencher) {
        let mut naive_trie = NaiveTrie::default();
        naive_trie.insert("test");

        b.iter(|| naive_trie.contains("test"));
    }

    #[bench]
    fn bench_contains_existing_vec_trie(b: &mut Bencher) {
        let mut vec_trie = VecTrie::default();
        vec_trie.insert("test");

        b.iter(|| vec_trie.contains("test"));
    }

    #[bench]
    fn bench_contains_existing_trimmed_hash_trie(b: &mut Bencher) {
        let mut trimmed_hash_trie = TrimmedHashTrie::default();
        trimmed_hash_trie.insert("test");

        b.iter(|| trimmed_hash_trie.contains("test"));
    }

    #[bench]
    fn bench_contains_existing_trimmed_vec_trie(b: &mut Bencher) {
        let mut trimmed_vec_trie = TrimmedVecTrie::default();
        trimmed_vec_trie.insert("test");

        b.iter(|| trimmed_vec_trie.contains("test"));
    }
}
