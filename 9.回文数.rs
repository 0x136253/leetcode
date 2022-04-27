/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        match s.as_bytes().windows(s.len()).find(|slice| {
            let iter = slice.iter();
            iter.clone().eq(iter.clone().rev())
        }){
            Some(_) => true,
            None => false,
        }
    }
}
// @lc code=end

