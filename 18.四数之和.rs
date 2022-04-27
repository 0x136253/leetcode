/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */

// @lc code=start
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let len = nums.len();
        if len < 4 {
            return res;
        }
        nums.sort_unstable();
        let min = nums[0] as i64 + nums[1] as i64 + nums[2] as i64 + nums[3] as i64;
        if (target as i64) < min {
            return res;
        }
        let max = nums[len - 1] as i64
            + nums[len - 2] as i64
            + nums[len - 3] as i64
            + nums[len - 4] as i64;
        if target as i64 > max {
            return res;
        }

        for i in 0..len - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let min = nums[i] as i64 + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64;
            if (target as i64) < min {
                continue;
            }
            let max =
                nums[i] as i64 + nums[len - 1] as i64 + nums[len - 2] as i64 + nums[len - 3] as i64;
            if target as i64 > max {
                continue;
            }

            for j in i + 1..len - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let (mut left, mut right) = (j + 1, len - 1);

                let min =
                    nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[left + 1] as i64;
                if (target as i64) < min {
                    continue;
                }
                let max =
                    nums[i] as i64 + nums[j] as i64 + nums[right] as i64 + nums[right - 1] as i64;
                if target as i64 > max {
                    continue;
                }

                while left < right {
                    let sum =
                        nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;

                    if sum < target as i64 {
                        left += 1;
                    } else if sum > target as i64 {
                        right -= 1;
                    } else {
                        res.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left += 1;
                        while nums[left] == nums[left - 1] && left < right {
                            left += 1;
                        }
                    }
                }
            }
        }
        res
    }
}
// @lc code=end
