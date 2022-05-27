/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut cnt = vec![];
        if n == 0{
            return res;
        }
        res.push('('.to_string());
        cnt.push((1,0));
        loop {
            let mut flag = true;
            let mut new_res = vec![];
            let mut new_cnt = vec![];
            for (i,val) in res.iter().enumerate(){
                let now_cnt = cnt[i];
                if now_cnt.0 < n{
                    flag = false;
                    let mut clone = val.clone();
                    clone.push('(');
                    new_res.push(clone);
                    new_cnt.push((now_cnt.0+1,now_cnt.1));
                }
                if now_cnt.1 < n && now_cnt.1+1 <= now_cnt.0{
                    flag = false;
                    let mut clone = val.clone();
                    clone.push(')');
                    new_res.push(clone);
                    new_cnt.push((now_cnt.0,now_cnt.1+1));
                }
            }
            if flag{
                break;
            }
            res = new_res;
            cnt = new_cnt;
        }
        res
    }
}
// @lc code=end

