use trie::bit_trie::BitTrie;
use trie::trimmed_vec_trie::TrimmedVecTrie;
use trie::{get_words_as_vec, get_3_letter_perms_as_vec};
use trie::trie::Trie;

fn main() {
    let mut trie = BitTrie::default();
    let mut trie2 = TrimmedVecTrie::default();
    let words = get_words_as_vec();
    let perms = get_3_letter_perms_as_vec();

    for word in words {
        trie.insert(&word);
        trie2.insert(&word);
    }

    for perm in perms.iter().filter(|perm| -> bool {trie.contains(perm) && !trie2.contains(perm)}) {
        println!("{}",perm);
    }
}
