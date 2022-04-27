/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut zigzags:Vec<_> = (0..num_rows)
            .chain((1..num_rows-1).rev())
            .cycle()
            .zip(s.chars())
            .collect();
        zigzags.sort_by_key(|&(row,_)| row);
        zigzags.into_iter()
            .map(|(_,c)| c)
            .collect()
    }
}
// @lc code=end

