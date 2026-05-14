impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut last: usize = nums.len()-1; // last idx
        if last == 0 {
            if nums[0] == target {
                return 0 as i32;
            } else {
                return -1 as i32;
            }
        } else if last == 1 {
            if nums[0] == target {
                return 0 as i32;
            } else if nums[1] == target {
                return 1 as i32;
            } else {
                return -1 as i32;
            }           
        }
        let mut first: usize = 0; // first idx

        if target > nums[last] || target < nums[first] {
            return -1 as i32;
        } 

        if target == nums[last] {
            return last as i32;
        }
        if target == nums[first] {
            return first as i32;
        }

        let mut mid: usize = last/2; // mid idx
        
        loop {
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target { // right
                if nums[mid+1] > target { // DNE
                    return -1 as i32;
                } else { // exists
                    first = mid;
                    mid += (last-first)/2;
                }
            } else { // left
                if nums[mid-1] < target {// DNE
                    return -1 as i32;
                } else { //exists
                    last = mid;
                    mid = (last-first)/2;
                }
            }
        }
    }
}
