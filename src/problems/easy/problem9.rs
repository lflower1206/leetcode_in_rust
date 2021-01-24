pub fn is_palindrome(input: i32) -> bool {
    if input < 0 {
        return false;
    }

    if input < 10 {
        return true;
    }

    let chars = input.to_string().chars().collect::<Vec<char>>();
    let len = chars.len();
    let mut left = 0;
    let mut right = len - 1;

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }

        left = left + 1;
        right = right - 1;
    }

    return true;
}

#[cfg(test)]
mod problem9 {
    use crate::problems::easy::problem9::is_palindrome;

    #[test]
    fn it_return_false_for_negative() {
        assert_eq!(is_palindrome(-1), false)
    }

    #[test]
    fn it_return_true_for_single_digit() {
        assert_eq!(is_palindrome(5), true)
    }

    #[test]
    fn it_return_false_for_123() {
        assert_eq!(is_palindrome(123), false)
    }

    #[test]
    fn it_return_true_for_131() {
        assert_eq!(is_palindrome(131), true)
    }
}
