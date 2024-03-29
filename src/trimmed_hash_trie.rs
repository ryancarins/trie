use ahash::RandomState;
/**
 * Trimmed trie
 *
 * For lack of a better name a trie that doesn't allocate extra memory for branches that don't exist using a HashMap instead
 * of an array
 */
use std::collections::HashMap;

use crate::trie::Trie;
#[derive(Debug)]
pub struct TrimmedHashTrie {
    root: TrimmedHashTrieNode,
}

#[derive(Debug)]
struct TrimmedHashTrieNode {
    //Determines if a word/sequence has ended
    end: bool,
    nodes: HashMap<u8, TrimmedHashTrieNode, RandomState>,
}

impl TrimmedHashTrieNode {
    pub fn new(end: bool) -> Self {
        Self {
            end,
            nodes: HashMap::default(),
        }
    }
}

impl Default for TrimmedHashTrie {
    fn default() -> Self {
        Self {
            root: TrimmedHashTrieNode::new(false),
        }
    }
}

impl Trie for TrimmedHashTrie {
    fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for character in word.chars() {
            let val = character as u8 - 97;
            current_node
                .nodes
                .entry(val)
                .or_insert_with(|| TrimmedHashTrieNode::new(false));
            current_node = current_node.nodes.get_mut(&val).unwrap();
        }
        current_node.end = true;
    }

    fn contains(&mut self, word: &str) -> bool {
        let mut current_node = &mut self.root;

        for character in word.chars() {
            let val = character as u8 - 97;
            if !current_node.nodes.contains_key(&val) {
                return false;
            }
            current_node = current_node.nodes.get_mut(&val).unwrap();
        }

        current_node.end
    }

    //Delete the string from the trie. If the string didn't exist to begin with
    // returns false, otherwise returns true
    // TODO: Clean up extra nodes
    fn delete(&mut self, word: &str) -> bool {
        let mut current_node = &mut self.root;

        for character in word.chars() {
            let val = character as u8 - 97;
            if !current_node.nodes.contains_key(&val) {
                return false;
            }
            current_node = current_node.nodes.get_mut(&val).unwrap();
        }

        current_node.end = false;
        true
    }
}
