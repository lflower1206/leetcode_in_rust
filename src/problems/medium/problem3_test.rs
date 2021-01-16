#[cfg(test)]
mod problem3 {
    use crate::problems::medium::problem3::length_of_longest_substring;

    #[test]
    fn it_returns_1_for_one_char() {
        assert_eq!(length_of_longest_substring(String::from("a")), 1)
    }

    #[test]
    fn it_returns_2_for_abba() {
        assert_eq!(length_of_longest_substring(String::from("abba")), 2)
    }

    #[test]
    fn it_returns_3_for_pwwkew() {
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3)
    }

    #[test]
    fn it_returns_3_for_repeat_characters() {
        assert_eq!(length_of_longest_substring(String::from("bbbb")), 1)
    }

    #[test]
    fn it_returns_3_for_tmmzuxt() {
        assert_eq!(length_of_longest_substring(String::from("tmmzuxt")), 5)
    }
}
