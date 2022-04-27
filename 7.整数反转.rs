/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    //
    pub fn reverse(x: i32) -> i32 {
        let mut y = x.abs().to_string();
        let mut y = y.as_bytes().to_vec();
        y.reverse();
        let mut ans = i32::from_str_radix(&String::from_utf8(y).unwrap(),10).unwrap_or(0);        
        match x {
            x if x < 0 => ans*-1,
            _ => ans
        }
    }
}
// @lc code=end

