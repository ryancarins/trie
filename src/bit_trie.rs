use std::default;

use bitvec::prelude::BitVec;

use crate::trie::Trie;

#[derive(Default)]
pub struct BitTrie {
    fields: BitVec,
    words: Vec<usize>
}

impl Trie for BitTrie {
    fn insert(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }

        let mut iterator = word.chars().into_iter();
        let mut index = iterator.next().unwrap() as usize - 97;
        if self.fields.len() <= index {
            self.fields.resize(index+1, false);
        }
        self.fields.set(index, true);
        for char in iterator {
            index= index*26 + char as usize - 97;
            if self.fields.len() <= index {
                self.fields.resize(index+1, false);
            }
            self.fields.set(index, true);
        }
        self.words.push(index);
    }

    fn contains(&mut self, word: &str) -> bool {
        return true;
    }

    fn delete(&mut self, word: &str) -> bool {
        return false;
    }
}