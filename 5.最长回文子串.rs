/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    //dp[i][j] = 
    pub fn longest_palindrome(s: String) -> String {
        let mut windows_size = s.len();
        while windows_size >0 {
            match s.as_bytes()
                   .windows(windows_size)
                   .find(|slice| {
                       let iter = slice.iter();
                       iter.clone().eq(iter.clone().rev())
                   }){
                       Some(slice) => return String::from_utf8(slice.to_vec()).unwrap_or("".to_string()),
                       None => windows_size-=1,
                   }
        }
        "".to_string()
    }
}
// @lc code=end

