/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ]
        .into_iter()
        .fold((s, 0), |(mut s, mut num), (base, unit)| {
            let mut t = 0;
            while s.starts_with(unit) {
                t+=1;
                s = String::from(&s[unit.len()..s.len()]);
            }
            (s,num + t*base)
        })
        .1
    }
}
// @lc code=end
