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
    let mut node1_current = l1;
    let mut node2_current = l2;
    let mut next_carry = 0;
    let mut node_head = Some(Box::new(ListNode::new(0)));
    let mut node_current = node_head.as_mut();

    while node1_current.is_some() || node2_current.is_some() || next_carry > 0 {
        let mut sum = next_carry;

        if let Some(node1) = node1_current {
            sum = sum + node1.val;
            node1_current = node1.next;
        }

        if let Some(node2) = node2_current {
            sum = sum + node2.val;
            node2_current = node2.next;
        }

        let (val, carry) = get_val_and_carry(&sum);

        next_carry = carry;

        if let Some(current) = node_current {
            current.next = Some(Box::new(ListNode::new(val)));
            node_current = current.next.as_mut();
        }
    }

    node_head.unwrap().next
}

pub fn get_val_and_carry(sum: &i32) -> (i32, i32) {
    (sum % 10, sum / 10)
}
