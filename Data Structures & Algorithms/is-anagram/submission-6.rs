use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // if s.len() != t.len() { return false; }
        
        // let mut counts = [0i32; 26];
        
        // for c in s.bytes() {
        //     counts[(c - b'a') as usize] += 1;
        // }
        // for c in t.bytes() {
        //     counts[(c - b'a') as usize] -= 1;
        // }
        
        // counts.iter().all(|&x| x == 0)
        //==================================================
        if s.len() != t.len() { return false; }
        let mut hash_s = HashMap::<char,u8>::new();
        let mut hash_t = HashMap::<char,u8>::new();

        let mut s_vec: Vec<char> = s.chars().collect();
        let mut t_vec: Vec<char> = t.chars().collect();

        for c in s_vec {
            if let Some(x) = hash_s.get_mut(&c) {
                *x = *x + 1;
            } else {
                hash_s.insert(c,1);
            }
        }
        for c in t_vec {
            if let Some(x) = hash_t.get_mut(&c) {
                *x = *x + 1;
            } else {
                hash_t.insert(c,1);
            }
        }
        if hash_s == hash_t {
            true
        } else {
            false
        }
        //==================================================
        //if s.len() != t.len() { return false; }
        // let mut chars: Vec<char> = s.chars().collect();
        // chars.sort_by(|a, b| b.cmp(a));

        // let mut chart: Vec<char> = t.chars().collect();
        // chart.sort_by(|a, b| b.cmp(a));

        // if chars == chart{
        //     true
        // }else{
        //     false
        // }
        //==================================================

    }
}