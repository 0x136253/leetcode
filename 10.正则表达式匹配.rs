/*
 * @lc app=leetcode.cn id=10 lang=rust
 *
 * [10] 正则表达式匹配
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect(); // 使用vec胖指针索引 较 s.chars().nth() 快得多。
        let p: Vec<char> = p.chars().collect();
        let match_c = |i, j| -> bool {
            // match函数直接上闭包
            i != 0 && (p[j - 1] == '.' || s[i - 1] == p[j - 1]) //因为返回值是bool类型 用&&比if更省事
        };
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1]; // 不能用数组 其初始化个数必须为常量
        dp[0][0] = true;
        (0..=s.len()).for_each(|i| {
            //迭代器较循环更易于优化
            (1..=p.len()).for_each(|j| {
                dp[i][j] = if p[j - 1] == '*' {
                    match_c(i, j - 1) && dp[i - 1][j] || dp[i][j - 2]
                } else {
                    match_c(i, j) && dp[i - 1][j - 1]
                };
            })
        });
        dp[s.len()][p.len()]
    }
}
// @lc code=end
