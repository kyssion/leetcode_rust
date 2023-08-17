use crate::Solution;
use std::cmp;

// 最长公共子序列
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp: Vec<i32> = vec![0;text2.len()];

        for (i, c) in text1.chars().enumerate() {
            let mut before_info = 0;
            for (j, c2) in text2.chars().enumerate() {
                let next_info = dp[j];
                dp[j] = if c == c2 { before_info + 1 } else { cmp::max(dp[j], if j == 0 { dp[j] } else { dp[j - 1] }) };
                before_info = next_info;
            }
        }
        return dp[dp.len() - 1];
    }

    // 等会学习一下高级语法糖
    pub fn longest_common_subsequence_ddd(text1: String, text2: String) -> i32 {
        let mut dp = vec![0; text2.len() + 1];
        let text1 = text1.chars().collect::<Vec<_>>();
        let text2 = text2.chars().collect::<Vec<_>>();
        for c in text1 {
            let mut pre = 0;
            for j in 1..=text2.len() {
                let tmp = dp[j];
                if text2[j - 1] == c {
                    dp[j] = pre + 1;
                } else {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                pre = tmp;
            }
        }
        dp[text2.len()]
    }
}
