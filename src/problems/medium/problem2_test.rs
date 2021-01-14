#[cfg(test)]
mod problem2 {
    use crate::problems::medium::problem2;
    use crate::problems::medium::problem2::ListNode;

    #[test]
    fn it_should_get_val_of_one_plus_one() {
        let l1 = problem2::ListNode { val: 1, next: None };
        let l2 = problem2::ListNode { val: 1, next: None };

        let boxed_l1 = Box::new(l1);
        let boxed_l2 = Box::new(l2);
        let result = problem2::add_two_numbers(Some(boxed_l1), Some(boxed_l2));

        match result {
            Some(result) => assert_eq!(result.val, 2),
            _ => {}
        }
    }

    #[test]
    fn it_should_get_val_of_six_plus_six() {
        let l1 = problem2::ListNode { val: 6, next: None };
        let l2 = problem2::ListNode { val: 6, next: None };

        let boxed_l1 = Box::new(l1);
        let boxed_l2 = Box::new(l2);
        let result = problem2::add_two_numbers(Some(boxed_l1), Some(boxed_l2));

        match result {
            Some(result) => {
                assert_eq!(result.val, 2);
                assert_eq!(result.next.unwrap().val, 1);
            }
            _ => {}
        }
    }

    #[test]
    fn it_should_get_val_of_twelve_plus_twelve() {
        let l1 = problem2::ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        };
        let l2 = problem2::ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        };

        let boxed_l1 = Box::new(l1);
        let boxed_l2 = Box::new(l2);
        let result = problem2::add_two_numbers(Some(boxed_l1), Some(boxed_l2));

        match result {
            Some(result) => {
                assert_eq!(result.val, 4);
                assert_eq!(result.next.unwrap().val, 2);
            }
            _ => {}
        }
    }

    #[test]
    fn it_should_get_val_of_101_plus_101() {
        let l1 = problem2::ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        };
        let l2 = problem2::ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        };

        let boxed_l1 = Box::new(l1);
        let boxed_l2 = Box::new(l2);
        let result = problem2::add_two_numbers(Some(boxed_l1), Some(boxed_l2));

        match result {
            Some(result) => {
                let second_node = result.next.unwrap();
                let third_node = second_node.next.unwrap();
                assert_eq!(result.val, 2);
                assert_eq!(second_node.val, 0);
                assert_eq!(third_node.val, 2);
            }
            _ => {}
        }
    }

    #[test]
    fn it_should_get_val_of_243_plus_564() {
        let l1 = problem2::ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        };
        let l2 = problem2::ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };

        let boxed_l1 = Box::new(l1);
        let boxed_l2 = Box::new(l2);
        let result = problem2::add_two_numbers(Some(boxed_l1), Some(boxed_l2));

        match result {
            Some(result) => {
                let second_node = result.next.unwrap();
                let third_node = second_node.next.unwrap();
                assert_eq!(result.val, 7);
                assert_eq!(second_node.val, 0);
                assert_eq!(third_node.val, 8);
            }
            _ => {}
        }
    }
}
