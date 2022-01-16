pub struct VecTrie {
    root: Vec<Option<bool>>,
}

impl VecTrie {
    pub fn default() -> Self {
        Self { root: Vec::new() }
    }

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
            if self.root[current_node + val] == None {
                self.root[current_node + val] = Some(false);
                current_node += val;
            }
        }
        self.root[current_node] = Some(true);
    }

    pub fn contains(&mut self, word: &str) -> bool {
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
            if self.root[current_node + val].is_some() {
                current_node += val;
            } else {
                return false;
            }
        }

        self.root[current_node].unwrap()
    }
}
