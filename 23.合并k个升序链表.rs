/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并K个升序链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_k_lists(mut xs: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // let i = xs
        //     .iter()
        //     .enumerate()
        //     .min_by_key(|(_, x)| x.as_ref().map_or(std::i32::MAX, |x| x.val))?
        //     .0;
        let mut index:usize = 0;
        let mut min = std::i32::MAX;
        for (i,val) in xs.iter().enumerate() {
            if val.is_some() &&val.as_ref().unwrap().val < min{
                min = val.as_ref().unwrap().val;
                index = i;
            }
        }
        if xs.len() == 0{
            return None;
        }
        let mut h = xs[index].take()?;
        xs[index] = h.next;
        h.next = Self::merge_k_lists(xs);
        Some(h)
    }
}
// @lc code=end
