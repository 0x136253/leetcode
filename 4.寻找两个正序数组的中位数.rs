/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
struct SortedVecsIter {
    vec1: Vec<i32>,
    vec2: Vec<i32>,
}

impl SortedVecsIter {
    fn len(&self) -> usize {
        self.vec1.len() + self.vec2.len()
    }
}


impl Iterator for SortedVecsIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        match (self.vec1.last(),self.vec2.last()) {
            (Some(n1),Some(n2)) if n1 >= n2 => self.vec1.pop(),
            (Some(_),Some(_))               => self.vec2.pop(),
            (Some(_), _)                     => self.vec1.pop(),
            (_, Some(_))                     => self.vec2.pop(),
            _                                => None,
        }
    }
}

impl Solution {
    // 3 -> 2
    //4 -> 2,3
    //5 -> 3
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
       let mut vecs_iter = SortedVecsIter {
           vec1: nums1,
           vec2: nums2,
       };
       let length = vecs_iter.len();
       let middle = length /2;
       if length%2 ==0 {
           let mut halves = vecs_iter.skip(middle -1);
           (halves.next().unwrap() as f64 + halves.next().unwrap() as f64) /2.0
       }else {
           vecs_iter.nth(middle).unwrap() as f64
       } 
    }
}
// @lc code=end

