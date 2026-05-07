impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min: i32 = prices[0];
        let mut consider: i32 = 0;
        let mut result: i32 = 0;

        for i in 0..prices.len() {
            // find out if it is considered as max
            if prices[i] > min {
                if prices[i] >= consider {
                    consider = prices[i];
                }
            }
            // change the min
            if prices[i] < min {
                min = prices[i];
                consider = 0; // reset to make sure not comparing current value with past one
            }
            if result < consider - min {
                result = consider - min;
            }
        }
        result
    }
}
