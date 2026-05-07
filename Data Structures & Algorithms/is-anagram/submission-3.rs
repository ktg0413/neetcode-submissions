impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));

        let mut chart: Vec<char> = t.chars().collect();
        chart.sort_by(|a, b| b.cmp(a));

        if chars == chart{
            true
        }else{
            false
        }
    }
}