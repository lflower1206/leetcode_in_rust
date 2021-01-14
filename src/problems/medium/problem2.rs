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
    let mut has_node1_next: bool;
    let mut has_node2_next: bool;
    let mut node1_next: Box<ListNode>;
    let mut node2_next: Box<ListNode>;
    let mut node_head = Some(Box::new(ListNode::new(val)));
    let mut node_current = node_head.as_mut();

    match node1.next {
        Some(node_next) => {
            has_node1_next = true;
            node1_next = node_next;
        }
        None => {
            has_node1_next = false;
            node1_next = fulfill_empty_node()
        }
    }

    match node2.next {
        Some(node_next) => {
            has_node2_next = true;
            node2_next = node_next;
        }
        None => {
            has_node2_next = false;
            node2_next = fulfill_empty_node()
        }
    }

    while has_node1_next || has_node2_next || next_carry > 0 {
        let (val, carry) = do_sum(&node1_next.val, &node2_next.val, &next_carry);
        next_carry = carry;

        if let Some(current) = node_current {
            current.next = Some(Box::new(ListNode::new(val)));
            node_current = current.next.as_mut();
        }

        match node1_next.next {
            Some(node_next) => {
                has_node1_next = true;
                node1_next = node_next;
            }
            None => {
                has_node1_next = false;
                node1_next = fulfill_empty_node()
            }
        }

        match node2_next.next {
            Some(node_next) => {
                has_node2_next = true;
                node2_next = node_next;
            }
            None => {
                has_node2_next = false;
                node2_next = fulfill_empty_node()
            }
        }
    }

    Some(node_head.unwrap())
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
