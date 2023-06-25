use std::collections::HashMap;
use crate::leetcode::Solution;

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

    pub fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.into_iter().enumerate(){
            // todo 搞明白这里为啥要使用引用方法
            if Some(&x) = map.get(&(target-num)){
                return vec![x , i as i32];
            }
            map.insert(num , i as i32)
        }
        return vec![]
    }
}
