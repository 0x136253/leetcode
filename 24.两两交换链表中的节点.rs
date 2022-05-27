/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = ListNode::new(0);
        let mut tail = &mut list.next;

        let mut temp = None;
        while let Some(mut node) = head.take() {
            head = node.next.take();

            match temp.take(){
                None => temp = Some(node),
                Some(temp) => {
                    node.next = Some(temp);
                    *tail = Some(node);
                    tail = &mut tail
                        .as_mut().unwrap().next
                        .as_mut().unwrap().next;
                }
            }
        }

        *tail = temp;
        list.next.take()
    }
}
// @lc code=end

