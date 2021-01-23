pub fn reverse(x: i32) -> i32 {
    let is_negative = x < 0;
    let input = x.to_string();
    let count = input.len();

    if count < 2 {
        return x;
    }

    let mut chars = input.chars().collect::<Vec<char>>();
    let (mut left, mut right) = if is_negative {
        get_indexes_for_negative(count)
    } else {
        get_indexes_for_positive(count)
    };

    loop {
        let tmp = chars[left];
        chars[left] = chars[right];
        chars[right] = tmp;

        left = if left == 0 { 0 } else { left - 1 };
        right = right + 1;

        if left > 0 || right < count {
            continue;
        }

        break;
    }

    match chars.iter().collect::<String>().parse::<i32>() {
        Ok(result) => result,
        _ => 0,
    }
}

fn get_indexes_for_positive(count: usize) -> (usize, usize) {
    // [0, 1, 2] [0, 1, 2, 3]
    let mid = count / 2;
    let left = mid - 1;
    let right = if count % 2 == 0 { mid } else { mid + 1 };

    (left, right)
}

fn get_indexes_for_negative(count: usize) -> (usize, usize) {
    // [-, 1, 2, 3] [-, 1, 2, 3, 4]
    let mid = count / 2;
    let left = if count % 2 == 0 { mid - 1 } else { mid };
    let right = mid + 1;

    (left, right)
}

#[cfg(test)]
mod problem7 {
    use crate::problems::easy::problem7::reverse;

    #[test]
    fn it_handle_single_digit() {
        assert_eq!(reverse(0), 0)
    }

    #[test]
    fn it_handle_2_digits() {
        assert_eq!(reverse(12), 21)
    }

    #[test]
    fn it_handle_3_digits() {
        assert_eq!(reverse(123), 321)
    }

    #[test]
    fn it_handle_1534236469() {
        assert_eq!(reverse(1534236469), 0)
    }

    #[test]
    fn it_handle_end_with_zero() {
        assert_eq!(reverse(1230), 321)
    }

    #[test]
    fn it_handle_negative_with_2_digits() {
        assert_eq!(reverse(-12), -21)
    }

    #[test]
    fn it_handle_negative_with_3_digits() {
        assert_eq!(reverse(-123), -321)
    }

    #[test]
    fn it_handle_negative_and_end_with_zero() {
        assert_eq!(reverse(-1230), -321)
    }
}
