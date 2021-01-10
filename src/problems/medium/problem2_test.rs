#[cfg(test)]
mod problem2 {
    use crate::problems::medium::problem2;

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
                assert_eq!(result.val, 1);
                assert_eq!(result.next.unwrap().val, 2);
            }
            _ => {}
        }
    }
}
