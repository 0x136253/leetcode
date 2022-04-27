/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let (mut x, mut y, mut z) = (0, 0, 0);
        for c in s.chars(){
            match c {
                '(' => x += 1,
                ')' => x -= 1,
                '[' => y += 1,
                ']' => y -= 1,
                '{' => z += 1,
                '}' => z -= 1,
                _  => (),
            };
        }

        if x == 0 && y == 0 && z == 0 {
            return true;
        }
        return false;
    }
}
// @lc code=end
