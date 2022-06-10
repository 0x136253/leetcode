/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut list = ListNode::new(0);
        let mut tail = &mut list.next;
        
        let mut pre = None;
        let mut cnt = 0;
        while let Some(mut now) = head.take() {
            head = now.next.take();

            match pre.take() {
                None => {
                    pre = Some(now),
                    cnt++;
                },
                Some(pre) => {
                    now.next = Some(pre),
                    cnt++;
                    if cnt == k{
                        *tail = pre;
                        
                    }else {
                        pre = Some(now);
                    }
                }
            }
        }
    }
}
// @lc code=end

