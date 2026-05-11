impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0; n];

        for i in 0..n {
            let mut count = 1;
            let mut j = i + 1;
            while j < n {
                if temperatures[j] > temperatures[i] {
                    break;
                }
                j += 1;
                count += 1;
            }
            res[i] = if j == n { 0 } else { count };
        }
        res
    }
}