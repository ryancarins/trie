use crate::trie::Trie;

/// Naive trie
///
/// A trie implemented as nodes containing a boolean to signify if a node is the end of a word
/// and an array of 26 pointers to other nodes with each index representing the character a-z

#[derive(Debug)]
pub struct NaiveTrie {
    root: Box<NaiveTrieNode>,
}

#[derive(Debug)]
struct NaiveTrieNode {
    //Determines if a word/sequence has ended
    end: bool,
    nodes: [Option<Box<NaiveTrieNode>>; 26],
}

impl NaiveTrieNode {
    pub fn new(end: bool) -> Self {
        Self {
            end,
            nodes: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
        }
    }
}

impl Default for NaiveTrie {
    fn default() -> Self {
        Self {
            root: Box::new(NaiveTrieNode::new(false)),
        }
    }
}

impl Trie for NaiveTrie {
    fn insert(&mut self, word: &str) {
        let mut current_node = &mut *self.root;
        for character in word.chars() {
            let val = character as usize - 97;
            if current_node.nodes[val].is_none() {
                current_node.nodes[val] = Some(Box::new(NaiveTrieNode::new(false)));
            }
            current_node = &mut *current_node.nodes[val].as_mut().unwrap();
        }
        current_node.end = true;
    }

    fn contains(&mut self, word: &str) -> bool {
        let mut current_node = &*self.root;
        for character in word.chars() {
            let val = character as usize - 97;
            if current_node.nodes[val].is_some() {
                current_node = &*current_node.nodes[val].as_ref().unwrap();
            } else {
                return false;
            }
        }

        current_node.end
    }

    ///Delete the string from the trie. If the string didn't exist to begin with
    /// returns false, otherwise returns true
    /// TODO: Clean up extra nodes
    fn delete(&mut self, word: &str) -> bool {
        let mut current_node = &mut *self.root;

        //Traverse to node to be deleted
        for character in word.chars() {
            if !character.is_ascii_alphabetic() {
                panic!("Lazy solution");
            }
            let val = character.to_ascii_lowercase() as usize - 97;
            if current_node.nodes[val].is_some() {
                current_node = &mut *current_node.nodes[val].as_mut().unwrap();
            } else {
                return false;
            }
        }
        current_node.end = false;
        //Check if we are on a leaf node, if we aren't short circuit and return true as we don't have
        //to delete extra nodes
        for i in 0..26 {
            if current_node.nodes[i].is_some() {
                return true;
            }
        }
        //TOOD: Clean up extra nodes

        true
    }
}
