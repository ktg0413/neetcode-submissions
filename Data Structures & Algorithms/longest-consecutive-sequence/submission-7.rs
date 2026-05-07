impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        nums.sort();
        nums.dedup();
        let mut first = nums[0];
        let mut current: i32 = 1;
        let mut result: i32 = 1;

        for &n in &nums {
            if n - first == 1 {
                current += 1;
                result = result.max(current);
            } else {
                current = 1;
            }
            first = n;
        }
        result
    }
}
