use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            if let Some(&x) = map.get(&(target - num)) {
                return vec![x, i as i32];
            }
            map.insert(num, i as i32);
        }
        vec![]
    }
}

pub struct Solution{}