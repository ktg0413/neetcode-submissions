impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded = String::new();
        for s in &strs {
            encoded.push_str(&format!("{}#{}", s.len(), s));
        }
        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut decoded = vec![];
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut j = i;
            while bytes[j] != b'#' {
                j += 1;
            }
            let len: usize = s[i..j].parse().unwrap();
            decoded.push(s[j + 1..j + 1 + len].to_string());
            i = j + 1 + len;
        }

        decoded
    }
}