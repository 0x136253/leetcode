#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn build_listnode(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = Some(Box::new(ListNode::new(0)));
    let mut head = &mut dummy;
    for i in 0..v.len() {
        head.next.as_mut().unwrap().val = v[i];
        if i != v.len() - 1 {
            head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
            head = head.next.as_mut().unwrap();
        }
    }
    dummy.next
}