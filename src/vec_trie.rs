/// Vec Trie
/// A trie built with a vector as the underlying data type.
/// O(k) time complexity
/// O(26^k) memory complexity (obviously a bad thing)
///
/// Much faster than the naive trie but takes up an enormous amount of memory
pub struct VecTrie {
    root: Vec<Option<bool>>,
}

impl VecTrie {
    /// Creates an empty trie
    pub fn default() -> Self {
        Self { root: Vec::new() }
    }

    /// Insert a string into the trie, if enough memory is already allocated we simply traverse and create each node that doesn't
    /// exist and set the last node to true
    /// If there isn't enough memory allocated we allocate the maximum needed for the word length (26^word_length - 1)
    pub fn insert(&mut self, word: &str) {
        let mut current_node = 0;
        //Find the max size of the trie given word length, check that the trie is already that large
        //if it isn't append enough elements to fix this
        let word_length = word.len();
        let max_size = 26usize.pow(word_length as u32) - 1;
        if max_size > self.root.len() {
            self.root
                .append(&mut vec![None; max_size - self.root.len()]);
        }

        for character in word.chars() {
            let val = character as usize - 97;
            if self.root[(current_node + 1) * 3 + val] == None {
                self.root[(current_node + 1) * 3 + val] = Some(false);
                current_node = (current_node + 1) * 3 + val;
            }
        }
        self.root[current_node] = Some(true);
    }

    /// Search the trie for the given string. Returns true if the trie contains the string
    /// O(k) time complexity
    pub fn contains(&mut self, word: &str) -> bool {
        let mut current_node = 0;
        //Find the max size of the trie given word length, check that the trie is already that large
        //if it isn't append enough elements to fix this

        for character in word.chars() {
            let val = character as usize - 97;
            if self.root[(current_node + 1) * 3 + val].is_some() {
                current_node = (current_node + 1) * 3 + val;
            } else {
                return false;
            }
        }

        self.root[current_node].unwrap()
    }

    ///Delete the string from the trie. If the string didn't exist to begin with
    /// returns false, otherwise returns true
    /// TODO: Clean up extra nodes
    pub fn delete(&mut self, word: &str) -> bool {
        let mut current_node = 0;

        for character in word.chars() {
            let val = character as usize - 97;
            if self.root[(current_node + 1) * 3 + val].is_some() {
                current_node = (current_node + 1) * 3 + val;
            } else {
                return false;
            }
        }

        self.root[current_node] = Some(false);
        true
    }
}
