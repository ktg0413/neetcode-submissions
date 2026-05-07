impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min: i32 = prices[0];
        let mut result: i32 = 0;

        for i in 0..prices.len() {
            // change the min
            if prices[i] < min {
                min = prices[i];
            }
            if result < prices[i] - min {
                result = prices[i] - min;
            }
        }
        result
    }
}
