impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s_vec = Vec::new();
        s.trim().to_lowercase().chars().map(|st| if st.is_alphanumeric() { s_vec.push(st)}).collect::<Vec<_>>();
        let vec: String = s_vec.iter().collect();
        let rev_vec: String = s_vec.iter().rev().collect();
        if vec == rev_vec {
            return true;
        } else {
            return false;
        }
    }
}
