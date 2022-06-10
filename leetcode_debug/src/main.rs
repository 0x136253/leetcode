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
        "{}",
        Solution::swap_pairs(build_listnode(vec![1,2,3,4,5])).unwrap()
    );
}

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = ListNode::new(0);
        let mut tail = &mut list.next;

        let mut pre = None;
        while let Some(mut now) = head.take() {
            head = now.next.take();

            match pre.take(){
                None => pre = Some(now),
                Some(pre) => {
                    now.next = Some(pre);
                    *tail = Some(now);
                    tail = &mut tail
                        .as_mut().unwrap().next
                        .as_mut().unwrap().next;
                }
            }

        }

        *tail = pre;
        list.next.take()
    }
}