/*
 * @lc app=leetcode.cn id=12 lang=rust
 *
 * [12] 整数转罗马数字
 */

// @lc code=start
const I: [&'static str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"]; //1~9
const X: [&'static str; 10] = ["","X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"]; //10~90
const C: [&'static str; 10] = ["","C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"]; //100~900
const M: [&'static str; 4] = ["","M", "MM", "MMM"]; //1000~3000

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let n  = num as usize;
        let mut s = M[n / 1000].to_string();
        s.push_str(C[(n % 1000) / 100]);
        s.push_str(X[(n % 100) / 10]);
        s.push_str(I[n % 10]);
        s
    }
}
// @lc code=end

