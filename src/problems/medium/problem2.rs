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
    let node1 = l1.unwrap();
    let node1_val = &node1.val;

    let node2 = l2.unwrap();
    let node2_val = &node2.val;

    let (val, carry) = do_sum(node1_val, node2_val, &0);

    let mut next_carry = carry;
    let mut node1_next = node1.next.unwrap_or(fulfill_empty_node());
    let mut node2_next = node2.next.unwrap_or(fulfill_empty_node());
    let mut result_node = ListNode::new(val);

    while node1_next.val > 0 || node2_next.val > 0 || next_carry > 0 {
        let (val, carry) = do_sum(&node1_next.val, &node2_next.val, &next_carry);
        next_carry = carry;

        let mut parent_node = ListNode::new(val);
        parent_node.next = Some(Box::new(result_node));
        result_node = parent_node;

        node1_next = node1_next.next.unwrap_or(fulfill_empty_node());
        node2_next = node2_next.next.unwrap_or(fulfill_empty_node());
    }

    Some(Box::new(result_node))
}

pub fn do_sum(val1: &i32, val2: &i32, carry: &i32) -> (i32, i32) {
    let sum = val1 + val2 + carry;
    let val = sum % 10;
    let carry = sum / 10;

    (val, carry)
}

pub fn fulfill_empty_node() -> Box<ListNode> {
    Box::new(ListNode::new(0))
}
