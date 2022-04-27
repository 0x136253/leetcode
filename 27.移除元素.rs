/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len(){
            if nums[i] != val {
                nums[ans] = nums[i];
                ans += 1;
            }
        }
        ans as i32
    }
}
// @lc code=end

