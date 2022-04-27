/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim();
        let mut ans = String::new();
        let mut flag_sign = true;
        for (i,val) in s.char_indices() {
            if (i == 0 && (val == '-' || val == '+')) || val.is_ascii_digit(){
                if i == 0 && val == '-' {
                    flag_sign = false;
                }
                ans.push(val);
            }else{
                break;
            }
        }
        let answer = i32::from_str_radix(&ans,10);
        if answer.is_ok(){
            answer.unwrap()
        }else {
            match flag_sign {
                true if ans.ends_with(|t:char|t.is_ascii_digit())=> 0x7fffffff,
                true if ans.starts_with("+-") => 0,
                true => 0,
                false if ans.starts_with("-+") => 0,
                false if ans == "-" => 0,
                false => -0x80000000,
            }  
        }
    }
}
// @lc code=end

