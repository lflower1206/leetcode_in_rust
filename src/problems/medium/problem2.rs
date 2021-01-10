#[warn(dead_code)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let val1 = l1.unwrap().val;
    let val2 = l2.unwrap().val;
    let node = ListNode::new(val1 + val2);

    Some(Box::new(node))
}
