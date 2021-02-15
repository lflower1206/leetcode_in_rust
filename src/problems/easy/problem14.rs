#[cfg(test)]
mod problem14 {
    struct Solution {}

    impl Solution {
        pub fn longest_common_prefix(strs: Vec<String>) -> String {
            if strs.is_empty() {
                return String::from("");
            }

            let mut prefix = strs.get(0).unwrap().as_str();

            for str in &strs[1..] {
                if prefix.is_empty() {
                    break;
                }

                loop {
                    match str.find(prefix) {
                        Some(position) => {
                            if position == 0 || prefix.is_empty() {
                                break;
                            }
                        }
                        _ => {}
                    }

                    let len = prefix.len() - 1;
                    prefix = &prefix[0..len];
                }
            }

            return String::from(prefix);
        }
    }

    #[test]
    fn it_return_empty_string() {
        let vec1 = vec![];
        assert_eq!(Solution::longest_common_prefix(vec1), String::from(""));
    }

    #[test]
    fn it_return_itself() {
        let vec1 = vec![String::from("abc")];
        assert_eq!(Solution::longest_common_prefix(vec1), String::from("abc"));
    }

    #[test]
    fn it_return_fl() {
        let vec1 = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        assert_eq!(Solution::longest_common_prefix(vec1), String::from("fl"));
    }

    #[test]
    fn it_return_empty_string_if_no_matched() {
        let vec1 = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        assert_eq!(Solution::longest_common_prefix(vec1), String::from(""));
    }
}
