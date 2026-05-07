use std::collections::{BinaryHeap,HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut map = HashMap::new();
        for &n in nums.iter() {
            let counter = map.entry(n).or_insert(0);
            *counter += 1;
        }
        for _i in 0..k {
            let mut max = 0;
            let mut m_num = 0;
            for (&num, &count) in map.iter() {
                if count > max {
                    max = count;
                    m_num = num;
                }
            }
            result.push(m_num);
            map.remove(&m_num);
        }
        result
    }
}
