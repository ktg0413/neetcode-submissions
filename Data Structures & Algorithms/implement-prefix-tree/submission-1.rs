struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            end_of_word: false,
        }
    }
}

struct PrefixTree {
    root: TrieNode,
}

impl PrefixTree {
    fn new() -> Self {
        Self { root: TrieNode::new() }
    }

    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        for c in word.bytes() {
            let i = (c - b'a') as usize;
            cur = cur.children[i].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        cur.end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = &self.root;
        for c in word.bytes() {
            let i = (c - b'a') as usize;
            match &cur.children[i] {
                Some(node) => cur = node,
                None => return false,
            }
        }
        cur.end_of_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = &self.root;
        for c in prefix.bytes() {
            let i = (c - b'a') as usize;
            match &cur.children[i] {
                Some(node) => cur = node,
                None => return false,
            }
        }
        true
    }
}