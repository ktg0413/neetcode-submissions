use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool { 
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' { continue; }
                
                let box_idx = (r / 3) * 3 + (c / 3);
                
                if !rows[r].insert(ch) { return false; }
                if !cols[c].insert(ch) { return false; }
                if !boxes[box_idx].insert(ch) { return false; }
            }
        }
        true
    }
}
