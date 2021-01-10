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
    let sum = val1 + val2;
    let carry = sum / 10;
    let val = sum % 10;

    let mut node;

    if carry > 0 {
        node = ListNode::new(carry);
        node.next = Some(Box::new(ListNode::new(val)));
    } else {
        node = ListNode::new(val);
    }

    Some(Box::new(node))
}
