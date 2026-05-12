impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position.into_iter()
            .zip(speed.into_iter())
            .collect();
        
        // 목표에 가까운 차부터 정렬
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        
        let mut stack: Vec<f64> = Vec::new();
        
        for (pos, spd) in cars {
            let time = (target - pos) as f64 / spd as f64;
            // 앞 차보다 오래 걸리면 → 따라잡지 못함 → 새 fleet
            if stack.last().map_or(true, |&top| time > top) {
                stack.push(time);
            }
            // 짧거나 같으면 → 앞 fleet에 합류 → push 안 함
        }
        
        stack.len() as i32
    }
}