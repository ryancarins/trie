#[derive(Debug)]
pub struct Trie {
    root: Box<TrieNode>,
}

#[derive(Debug)]
struct TrieNode {
    //Determines if a word/sequence has ended
    end: bool,
    nodes: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
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

impl Trie {
    pub fn default() -> Self {
        Self {
            root: Box::new(TrieNode::new(false)),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut *self.root;
        for character in word.chars() {
            if !character.is_ascii_alphabetic() {
                panic!("Lazy solution");
            }
            let val = character.to_ascii_lowercase() as usize - 97;
            if current_node.nodes[val].is_none() {
                current_node.nodes[val] = Some(Box::new(TrieNode::new(false)));
            }
            current_node = &mut *current_node.nodes[val].as_mut().unwrap();
        }
        current_node.end = true;
    }
}