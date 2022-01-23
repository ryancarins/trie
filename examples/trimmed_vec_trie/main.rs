use trie::trimmed_vec_trie::TrimmedVecTrie;
use trie::get_words_as_vec;

fn main() {
    let words = get_words_as_vec();
    let mut trie = TrimmedVecTrie::default();

    for word in words {
        trie.insert(&word);
    }
}
