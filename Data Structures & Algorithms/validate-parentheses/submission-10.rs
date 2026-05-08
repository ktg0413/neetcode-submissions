impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec: Vec<char> = vec![];
        let mut state: bool = false;
        for ch in s.chars() {
            if ch == '(' {
                vec.push(ch); // (
            } else if ch == '{' {
                vec.push(ch); // {
            } else if ch == '[' {
                vec.push(ch); // [
            }
            if ch == ')' {
                if vec.pop() == Some('(') {state = true;} else {return false;}
            } else if ch == '}' {
                if vec.pop() == Some('{') {state = true;} else {return false;}   
            } else if ch == ']' {
                if vec.pop() == Some('[') {state = true;} else {return false;}
            }
        } 
        if vec.len() > 0 {state = false;}
        state
    }
}
