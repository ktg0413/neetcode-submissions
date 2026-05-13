impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        
        for i in 0..numbers.len() {
            let comp: i32 = numbers[i];
            let n = target - comp;
            for j in 1..numbers.len() {
                if numbers[j] == n {
                    result.push(i as i32 + 1);
                    result.push(j as i32 + 1);
                }
            }
        }
        result
    }
}
