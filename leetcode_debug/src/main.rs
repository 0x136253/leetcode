/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] 两数相除
 */

// @lc code=start
mod list_node;
use crate::list_node::*;

// use leetcode_debug::list_node::*;
struct Solution {}

fn main() {
    print!(
        "{:?}",
        Solution::is_valid("([)]".to_string())
    );
}

/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
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
