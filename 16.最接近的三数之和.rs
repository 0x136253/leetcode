/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut min_res = 0x3fff_ffff;
        let mut res = 0x3fff_ffff;
        let mut nums = nums.clone();
        nums.sort();

        for i in (0..nums.len()) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];


                let sum_c = (sum - target).abs();
                if sum_c < min_res {
                    min_res = sum_c;
                    res = sum;
                }
                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else{
                    left += 1;
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }
        res
    }
}
// @lc code=end
