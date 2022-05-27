/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut queue = vec![];
        for i in s.chars() {
            match i {
                '(' => queue.push(')'),
                '[' => queue.push(']'),
                '{' => queue.push('}'),
                ')' | ']' | '}' if Some(i) != queue.pop() => return false,
                _ => ()
            }
        }
        return queue.is_empty();
    }
}
// @lc code=end
