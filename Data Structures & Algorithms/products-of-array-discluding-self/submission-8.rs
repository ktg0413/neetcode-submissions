impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product: i32 = 1;
        let mut zero: bool = false;
        let mut zcount: i32 = 0;
        let mut sum: i32 = 0;
        for &num in nums.iter() {
            if num != 0 {
                product = product * num;
            } else {
                zero = true;
                zcount += 1;
            }
            sum += num;
        }
        if sum == 0 && product == 1 && zero{
            product = 0;
        }
        let mut result: Vec<i32> = vec![];
        for &num in nums.iter() {
            if zero {
                if zcount >= 2 {
                    result.push(0);
                }else if num == 0 {
                    result.push(product);
                } else {
                    result.push(0);
                }
            } else {
                result.push(product/num);
            }
        }
        result
    }
}
