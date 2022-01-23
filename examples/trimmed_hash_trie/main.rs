use trie::trimmed_hash_trie::TrimmedHashTrie;
use trie::get_words_as_vec;

fn main() {
    let words = get_words_as_vec();
    let mut trie = TrimmedHashTrie::default();

    for word in words {
        trie.insert(&word);
    }
}
