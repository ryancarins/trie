use bitvec::prelude::BitVec;
use std::collections::HashSet;

use crate::trie::Trie;

#[derive(Default)]
pub struct BitTrie {
    pub fields: BitVec,
    pub words: HashSet<usize>
}

impl Trie for BitTrie {
    fn insert(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }

        let mut iterator = word.chars().into_iter();
        let mut index = iterator.next().unwrap() as usize - 96;
        if self.fields.len() <= index {
            self.fields.resize(index+1, false);
        }
        self.fields.set(index, true);
        for char in iterator {
            index= index*26 + char as usize - 96;
            if self.fields.len() <= index {
                self.fields.resize(index+1, false);
            }
            self.fields.set(index, true);
        }
        self.words.insert(index);
    }

    fn contains(&mut self, word: &str) -> bool {
        let mut index = 0;
        for char in word.chars().map(|x| -> usize {x as usize -96}) {
            index = index*26 + char;
            if self.fields.len() <= index {
                return false;
            }

            if !self.fields[index] {
                return false;
            }
        }

        return self.words.contains(&index);
    }

    fn delete(&mut self, word: &str) -> bool {
        return false;
    }
}