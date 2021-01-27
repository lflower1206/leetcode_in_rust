use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut result = 0;

    while left < right {
        let distance = (right - left) as i32;
        let height_left = height[left];
        let height_right = height[right];
        let area = cmp::min(height_left, height_right) * distance;

        result = cmp::max(result, area);

        if height_left < height_right {
            left = left + 1;
        } else {
            right = right - 1;
        }
    }

    result
}

#[cfg(test)]
mod problem11 {
    use crate::problems::medium::problem11::max_area;

    #[test]
    fn it_should_return_area() {
        assert_eq!(max_area(vec![3, 4]), 3)
    }

    #[test]
    fn it_should_return_max_area() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
    }
}
