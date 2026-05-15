struct PrefixTree {
    arr: Vec<String>,
}

impl PrefixTree {
    pub fn new() -> Self {
        PrefixTree {
            arr: vec![],
        }
    }

    pub fn insert(&mut self, word: String) {
        self.arr.push(word);
    }

    pub fn search(&self, word: String) -> bool {
        for s in self.arr.iter() {
            if s == &word {
                return true;
            }
        }
        false
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let prelen = prefix.chars().count();
        for s in self.arr.iter() {
            let x = s.chars().take(prelen).collect::<String>();
            if prefix == x {
                return true;
            }
        }
        false
    }
}
