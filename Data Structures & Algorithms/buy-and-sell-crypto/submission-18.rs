impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut result: i32 = 0;

        for &price in &prices {
            min = min.min(price);
            result = result.max(price-min);
        }
        result
    }
}
