/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] 两数相除
 */

// @lc code=start
// struct Solution{}
// fn main(){
//     print!("{}",Solution::divide(-2147483648,-1));
// }
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let signs = (
            if dividend >=0 {1} else {-1},
            if divisor >=0 {1} else {-1},
        );
        let sign = match signs.0+signs.1 {
            0 => true,
            _ => false,
        };

        let mut n = Self::recursive_divide((dividend as i64).abs(), (divisor as i64).abs());
        
        if sign {
            n = (!n) + 1;
        }

        if n > i32::MAX as i64 {
            return i32::MAX;
        } else if n < i32::MIN as i64 {
            return i32::MIN;
        } else {
            return n as i32;
        }
    }

    pub fn recursive_divide(dividend: i64, divisor: i64) -> i64{
        if dividend < divisor {
            return 0;
        }

        let (mut q,mut  r) = Self::rdiv(dividend,divisor);
        while r >= divisor {
            let (temp_q,temp_r) = Self::rdiv(r,divisor);
            q+=temp_q;
            r = temp_r;
        }
        return q;
    }

    pub fn rdiv(dividend: i64,divisor: i64) -> (i64,i64) {
        if dividend - divisor < divisor {
            return (1,dividend - divisor);
        } else {
            let (q, r) = Self::rdiv(dividend >> 1, divisor);
            return (q << 1, (r << 1) + (dividend & 1));
        }
    }
}
// @lc code=end

