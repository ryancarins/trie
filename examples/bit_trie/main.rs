use trie::bit_trie::BitTrie;
use trie::get_words_as_vec;
use trie::trie::Trie;

fn main() {
    let words = get_words_as_vec();
    let mut trie = BitTrie::default();

    for word in words {
        trie.insert(&word);
    }
}
