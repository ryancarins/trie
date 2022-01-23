use trie::naive_trie::NaiveTrie;
use trie::get_words_as_vec;

fn main() {
    let words = get_words_as_vec();
    let mut trie = NaiveTrie::default();

    for word in words {
        trie.insert(&word);
    }

    let words = trie.countdown_search("ugaoilrim");
    for word in words {
        println!("{}", word);
    }
}
