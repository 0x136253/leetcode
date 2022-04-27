/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i:usize = 0;
        let mut j:usize = height.len()-1;
        while i<j{
            ans = cmp::max(ans, (j-i) as i32*cmp::min(height[i], height[j]));
            if height[i] < height[j] {
                i+=1;
            }else {
                j-=1;
            }
        }
        ans
    }
}
// @lc code=end

