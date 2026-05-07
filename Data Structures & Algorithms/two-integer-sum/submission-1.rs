use std::collections::HashSet;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut comp:HashSet<i32> = HashSet::new();
        let mut result:Vec<i32> = Vec::new();
        for i in nums.iter() {
            comp.insert(*i);
        }
        let mut count:u8 = 0;
        let mut temp_idx = 0;
        for (inx,i) in nums.into_iter().enumerate() {
            if !comp.insert(target - i) {
                if target - i == i {
                    if count == 0 {
                        count += 1;
                        temp_idx = inx;
                    } else {
                        result.push(temp_idx.try_into().unwrap());
                        result.push(inx.try_into().unwrap());
                    }
                } else {
                    result.push(inx.try_into().unwrap());
                }
            }
        }
        result
    }
}
