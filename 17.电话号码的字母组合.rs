/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {  
        let mut res:Vec<String> = Vec::new();
        let mut nums = digits.chars();
        match nums.next() {
            Some(head) => {
                for c in Self::num_to_letter(head){
                    let mut s = String::new();
                    s.push(c);
                    res.push(s);
                }
            },
            None => return res,
        }

        while let Some(num) = nums.next() {
            let mut new_res = vec![];
            for val in res.iter(){
                for c in Self::num_to_letter(num){
                    let mut clone = val.clone();
                    clone.push(c);
                    new_res.push(clone);
                }
            }
            res = new_res;
        }
        res
    }

    fn num_to_letter(number: char) -> Vec<char> {
        match number {
            '2' => vec!['a','b','c'], 
            '3' => vec!['d','e','f'], 
            '4' => vec!['g','h','i'], 
            '5' => vec!['j','k','l'], 
            '6' => vec!['m','n','o'], 
            '7' => vec!['p','q','r','s'], 
            '8' => vec!['t','u','v'], 
            '9' => vec!['w','x','y','z'],
            _ => vec![], 
        }
    }
}
// @lc code=end

