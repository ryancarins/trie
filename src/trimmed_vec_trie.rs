/**
 * Trimmed trie
 *
 * For lack of a better name a trie that doesn't allocate extra memory for branches that don't exist using a vector instead of an array
 * to add branches. Looping over this may be faster than using the hashmap as the vector has a maximum size of 26 elements
 */
#[derive(Debug, Clone)]
pub struct TrimmedVecTrie {
    root: TrimmedVecTrieNode,
}

#[derive(Debug, Clone)]
struct TrimmedVecTrieNode {
    //Determines if a word/sequence has ended
    end: bool,
    nodes: Vec<(u8, TrimmedVecTrieNode)>,
}

impl TrimmedVecTrieNode {
    pub fn new(end: bool) -> Self {
        Self {
            end,
            nodes: Vec::new(),
        }
    }
}

impl TrimmedVecTrie {
    pub fn default() -> Self {
        Self {
            root: TrimmedVecTrieNode::new(false),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        let mut next_node = 0; //Set this as 0 because the compiler can't guarantee this is initialised. I can however

        for character in word.chars() {
            let val = character as u8 - 97;
            let mut found = false;
            for (index, (i, _)) in current_node.nodes.iter().enumerate() {
                if *i == val {
                    next_node = index;
                    found = true;
                    break;
                }
            }
            if !found {
                current_node
                    .nodes
                    .push((val, TrimmedVecTrieNode::new(false)));
                next_node = current_node.nodes.len() - 1;
            }
            current_node = &mut current_node.nodes[next_node].1;
        }
        current_node.end = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        let mut next_node = 0; //Set this as 0 because the compiler can't guarantee this is initialised. I can however

        for character in word.chars() {
            let val = character as u8 - 97;
            let mut found = false;
            for (index, (i, _)) in current_node.nodes.iter().enumerate() {
                if *i == val {
                    next_node = index;
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
            current_node = &current_node.nodes[next_node].1;
        }
        current_node.end
    }

    //Delete the string from the trie. If the string didn't exist to begin with
    // returns false, otherwise returns true
    // TODO: Clean up extra nodes
    pub fn delete(&mut self, word: &str) -> bool {
        let mut current_node = &mut self.root;
        let mut next_node = 0; //Set this as 0 because the compiler can't guarantee this is initialised. I can however

        for character in word.chars() {
            let val = character as u8 - 97;
            let mut found = false;
            for (index, (i, _)) in current_node.nodes.iter().enumerate() {
                if *i == val {
                    next_node = index;
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
            current_node = &mut current_node.nodes[next_node].1;
        }

        current_node.end = false;
        true
    }
}
